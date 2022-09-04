
use rand::prelude::Rng;
use super::math::Vector2;

pub struct Cluster {
  pub data: Vec<Vector2>,
  pub centroids: Vec<Vector2>,
  pub pivot_rate: f64,
}

impl Default for Cluster {
  fn default() -> Self {
    Self {
      data: Vec::<Vector2>::new(),
      centroids: Vec::<Vector2>::new(),
      pivot_rate: 0.8,
    }
  }
}

impl Cluster {
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
        y: rng.gen_range(0.0..1.0)
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
        y: theta.cos() * 0.5 + 0.5
      });
    }
  }

  pub fn fit(&mut self, max_loop: usize, goal_diff: f64) {
    for _n in 0..max_loop {
      if self.fit_once() <= goal_diff {
        break;
      }
    }
  }

  pub fn fit_once(&mut self) -> f64 {
    let data_len = self.data.len();
    let centroid_len = self.centroids.len();

    let mut distances = vec![vec![0.0; data_len]; centroid_len];
    let mut distance_sum = vec![0.0; centroid_len];
    let mut distance_max = vec![0.0; centroid_len];

    // Set distances between data and centroids
    for n in 0..centroid_len {
      for m in 0..data_len {
        distances[n][m] = self.centroids[n].distance(&self.data[m]);

        if distances[n][m] > distance_max[n] {
          distance_max[n] = distances[n][m];
        }
      }

      for m in 0..data_len {
        distances[n][m] = distance_max[n] - distances[n][m];
      }

      distance_sum[n] = distances[n].iter().sum();
    }

    // Save previous centroids position
    let prev_centroids = self.centroids.to_vec();

    // Update centroids position
    for n in 0..centroid_len {
      let mut weights = vec![0.0; data_len];
      for m in 0..data_len {
        weights[m] = distances[n][m] / distance_sum[n];
      }

      let mut sorted_weights: Vec<f64> = weights.to_vec();
      sorted_weights.sort_by(|a, b| a.partial_cmp(b).unwrap());

      let pivot_index: usize = (data_len as f64 * self.pivot_rate) as usize;
      let pivot = sorted_weights[pivot_index];

      for m in 0..data_len {
        if weights[m] > pivot {
          self.centroids[n].x += (self.data[m].x - self.centroids[n].x) * weights[m];
          self.centroids[n].y += (self.data[m].y - self.centroids[n].y) * weights[m];
        }
      }
    }

    // Calc centroid diffence
    let mut centroid_distance_diff = 0.0;
    for n in 0..centroid_len {
      centroid_distance_diff += self.centroids[n].distance(&prev_centroids[n]);
    }
    centroid_distance_diff /= centroid_len as f64;
    centroid_distance_diff
  }
}
