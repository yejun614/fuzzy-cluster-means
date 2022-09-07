#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release

use eframe;
use egui;
use egui::plot::{Plot, PlotPoints, Points, MarkerShape};

// use super::cluster::Cluster;
use super::fcm_cluster::FcmCluster;
use super::math::Vector2;
use super::read_csv;

pub struct App {
    centroid_len: usize,
    dataset: Vec<Vec<String>>,
    dataset_file_path: String,
    dataset_skip_header: bool,
    dataset_column_x: usize,
    dataset_column_y: usize,
    centroid_placement_method: usize,
    cluster_max_loop: usize,
    cluster_goal_diff: f64,
    prev_centroids: Vec<Vector2>,
    current_turn: usize,
    pub cluster: FcmCluster,
}

impl Default for App {
    fn default() -> Self {
        Self {
            centroid_len: 2,
            dataset: Vec::<Vec<String>>::new(),
            dataset_file_path: "".to_string(),
            dataset_skip_header: true,
            dataset_column_x: 0,
            dataset_column_y: 0,
            centroid_placement_method: 0,
            cluster_max_loop: 300,
            cluster_goal_diff: 0.0,
            prev_centroids: Vec::<Vector2>::new(),
            current_turn: 0,
            cluster: FcmCluster::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Fuzzy Cluster Means");

                ui.collapsing("Dataset", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("File Path (CSV)");
                        ui.text_edit_singleline(&mut self.dataset_file_path);

                        if ui.button("Open").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.dataset_file_path = path.display().to_string();

                                self.dataset = read_csv(&self.dataset_file_path);
                                self.dataset_column_x = 0;
                                self.dataset_column_y = 0;
                            }
                        }
                    });
                    ui.add_space(10.0);

                    ui.checkbox(&mut self.dataset_skip_header, "Skip Header");
                    ui.add_space(10.0);

                    if self.dataset.len() > 0 {
                        ui.label("Columns");
                        let column_len = self.dataset[0].len();

                        egui::ComboBox::from_label("Column X")
                            .selected_text(format!(
                                "{} ({})",
                                self.dataset[0][self.dataset_column_x], self.dataset_column_x
                            ))
                            .show_ui(ui, |ui| {
                                for n in 0..column_len {
                                    ui.selectable_value(
                                        &mut self.dataset_column_x,
                                        n,
                                        format!("{} ({})", self.dataset[0][n], n),
                                    );
                                }
                            });

                        egui::ComboBox::from_label("Column Y")
                            .selected_text(format!(
                                "{} ({})",
                                self.dataset[0][self.dataset_column_y], self.dataset_column_y
                            ))
                            .show_ui(ui, |ui| {
                                for n in 0..column_len {
                                    ui.selectable_value(
                                        &mut self.dataset_column_y,
                                        n,
                                        format!("{} ({})", self.dataset[0][n], n),
                                    );
                                }
                            });
                    }
                });

                ui.collapsing("Parameters", |ui| {
                    egui::Grid::new("parameter_grid").show(ui, |ui| {
                        ui.label("Centroid Len");
                        ui.add(egui::Slider::new(&mut self.centroid_len, 1..=20));
                        ui.end_row();

                        ui.label("Pivot Rate");
                        ui.add(egui::Slider::new(&mut self.cluster.pivot_rate, 0.01..=0.99));
                        ui.end_row();

                        let centroid_placement_methods = ["Random", "Improved"];

                        ui.label("Centroid Placement Method");
                        egui::ComboBox::from_label("")
                            .selected_text(
                                centroid_placement_methods[self.centroid_placement_method],
                            )
                            .show_ui(ui, |ui| {
                                for n in 0..2 {
                                    ui.selectable_value(
                                        &mut self.centroid_placement_method,
                                        n,
                                        centroid_placement_methods[n],
                                    );
                                }
                            });
                        ui.end_row();
                    });

                    ui.separator();
                    ui.end_row();

                    egui::Grid::new("parameter_grid2").show(ui, |ui| {
                        ui.label("Auto Fit - Max Loop");
                        ui.add(egui::Slider::new(&mut self.cluster_max_loop, 1..=500));
                        ui.end_row();

                        ui.label("Auto Fit - Goal Diff");
                        ui.add(egui::Slider::new(&mut self.cluster_goal_diff, 0.0..=1.0));
                        ui.end_row();
                    });
                });
                ui.add_space(10.0);

                if self.dataset.len() > 0 {
                    if ui.button("Cluster Reset").clicked() {
                        self.current_turn = 0;
                        self.prev_centroids = vec![Vector2::new(0.0, 0.0); self.centroid_len];
                        let start_index = if self.dataset_skip_header { 1 } else { 0 };

                        self.cluster.data = (start_index..self.dataset.len())
                            .map(|i| Vector2 {
                                x: self.dataset[i][self.dataset_column_x]
                                    .parse::<f64>()
                                    .unwrap(),
                                y: self.dataset[i][self.dataset_column_y]
                                    .parse::<f64>()
                                    .unwrap(),
                            })
                            .collect();

                        self.cluster.data_normalize();

                        if self.centroid_placement_method == 0 {
                            self.cluster.set_random_centroids(self.centroid_len);
                        } else {
                            self.cluster.set_improved_centroids(self.centroid_len);
                        }
                    }

                    if ui.button("Fit Auto").clicked() {
                        self.cluster
                            .fit(self.cluster_max_loop, self.cluster_goal_diff);
                    }

                    if ui.button("Fit Once").clicked() {
                        self.current_turn += 1;

                        if self.current_turn % 2 == 0 {
                            self.cluster.fit_once();
                        } else {
                            self.prev_centroids = self.cluster.centroids.clone();
                        }
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let data_len = self.cluster.data.len();
            let centroid_len = self.cluster.centroids.len();

            let centroids: PlotPoints = (0..centroid_len)
                .map(|i| [self.cluster.centroids[i].x, self.cluster.centroids[i].y])
                .collect();
            let centroid_points = Points::new(centroids)
                .shape(MarkerShape::Asterisk)
                .radius(8.0)
                .name("Centroid");

            let mut data: Vec<Vec<Vector2>> = vec![vec![Vector2::new(0.0, 0.0); 0]; centroid_len];

            for n in 0..data_len {
                let mut distance = f64::MAX;
                let mut centroid_index = 0;

                for m in 0..centroid_len {
                    let current = self.cluster.data[n].distance(&self.prev_centroids[m]);

                    if current < distance {
                        distance = current;
                        centroid_index = m;
                    }
                }

                data[centroid_index].push(self.cluster.data[n]);
            }

            Plot::new("main_plot")
                .show(ui, |plot_ui| {
                    for n in 0..centroid_len {
                        let plot_points: PlotPoints = (0..data[n].len())
                            .map(|i| [data[n][i].x, data[n][i].y])
                            .collect();
                        let points = Points::new(plot_points).radius(5.0);

                        plot_ui.points(points);
                    }

                    plot_ui.points(centroid_points);
                });
        });
    }
}

impl App {
    pub fn run_native(self) {
        let options = eframe::NativeOptions::default();

        eframe::run_native(
            "Fuzzy Cluster Means",
            options,
            Box::new(|_cc| Box::new(self)),
        );
    }
}
