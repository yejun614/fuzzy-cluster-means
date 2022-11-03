use super::math::Vector2;
use rand::prelude::Rng;

pub struct FcmCluster {
    pub data: Vec<Vector2>,
    pub centroids: Vec<Vector2>,
    pub pivot_rate: f64,
}

impl Default for FcmCluster {
    fn default() -> Self {
        Self {
            data: Vec::<Vector2>::new(),
            centroids: Vec::<Vector2>::new(),
            pivot_rate: 0.8,
        }
    }
}

impl FcmCluster {
    pub fn data_normalize(&mut self) {
        let data_len = self.data.len();
        let min_pos = Vector2::min(data_len, |n| self.data[n]);
        let mut max_pos = Vector2::max(data_len, |n| self.data[n]);

        max_pos.x -= min_pos.x;
        max_pos.y -= min_pos.y;

        for n in 0..data_len {
            self.data[n].x = (self.data[n].x - min_pos.x) / max_pos.x;
            self.data[n].y = (self.data[n].y - min_pos.y) / max_pos.y;
        }
    }

    pub fn set_random_centroids(&mut self, centroid_len: usize) {
        self.centroids.clear();
        let mut rng = rand::thread_rng();

        for _n in 0..centroid_len {
            let centroid = Vector2 {
                x: rng.gen_range(0.0..1.0),
                y: rng.gen_range(0.0..1.0),
            };

            self.centroids.push(centroid);
        }
    }

    pub fn set_improved_centroids(&mut self, centroid_len: usize) {
        self.centroids.clear();

        for n in 0..centroid_len {
            let theta = n as f64 * 2.0 * std::f64::consts::PI / centroid_len as f64;

            self.centroids.push(Vector2 {
                x: theta.sin() * 0.5 + 0.5,
                y: theta.cos() * 0.5 + 0.5,
            });
        }
    }

    pub fn fit(&mut self, max_loop: usize, goal_diff: f64) -> usize {
        let mut count = 0;
        for _n in 0..max_loop {
            count += 1;

            if self.fit_once() <= goal_diff {
                break;
            }
        }

        count
    }

    pub fn fit_once(&mut self) -> f64 {
        let m_value = 2.0;

        let data_len = self.data.len();
        let centroid_len = self.centroids.len();
        let prev_centroids = self.centroids.clone();

        let mut weights = vec![vec![0.0; data_len]; centroid_len];

        for c in 0..centroid_len {
          for v in 0..data_len {
            for c2 in 0..centroid_len {
              weights[c][v] += ((self.data[v].distance(&self.centroids[c])) / (self.data[v].distance(&self.centroids[c2]))).powf(2.0 / (m_value - 1.0));
            }
            weights[c][v] = weights[c][v].powf(-1.0);
          }
        }

        for c in 0..centroid_len {
          let mut weight_data = Vector2::new(0.0, 0.0);
          let mut weight_sum = 0.0;

          for v in 0..data_len {
            weight_data.x += weights[c][v].powf(m_value) * self.data[v].x;
            weight_data.y += weights[c][v].powf(m_value) * self.data[v].y;
            weight_sum += weights[c][v].powf(m_value);
          }

          self.centroids[c].x = weight_data.x / weight_sum;
          self.centroids[c].y = weight_data.y / weight_sum;
        }

        let mut diff = 0.0;

        for c in 0..centroid_len {
          diff += self.centroids[c].distance(&prev_centroids[c]);
        }

        diff /= centroid_len as f64;

        diff
    }
}
