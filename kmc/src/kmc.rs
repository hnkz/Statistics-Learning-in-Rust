extern crate rand;
use self::rand::Rng;
use std::process;

#[path = "standard.rs"]
mod stardard;

#[derive(Debug)]
pub struct KMeans {
    n_clusters: i32,
    max_iter: i32,
    points: Vec<(f64, f64)>,
    labels: Vec<i32>,
    labels_count: Vec<i64>,
    cluster_centers: Vec<(f64, f64)>,
}
impl KMeans {
    pub fn new(n_clusters: i32, max_iter: i32) -> KMeans {
        KMeans {
            n_clusters: n_clusters,
            max_iter: max_iter,
            points: Vec::new(),
            labels: Vec::new(),
            labels_count:Vec::new(),
            cluster_centers: Vec::new(),
        }
    }

    pub fn set_data(&mut self, points: &Vec<(f64, f64)>) {
        self.points = points.to_vec();
    }

    pub fn predict_f(&mut self) {
        let mut rng = rand::thread_rng();
        let mut labels: Vec<i32> = Vec::new();
        let mut labels_count: Vec<i64> = Vec::new();
        let points_length = self.points.len();

        for i in 0..self.n_clusters {
            labels_count.push(0i64);
        }
        
        for i in 0..points_length {
            let n = rng.gen_range(0, self.n_clusters) as i32;
            labels.push(n);
            labels_count[n as usize] += 1;
        }
        self.labels = labels;
        self.labels_count = labels_count;

        for i in 0..self.max_iter {
            self.calc_center();
            if self.calc_next_label() {
                break;
            }
        }
        
        for (point, label) in self.points.iter().zip(self.labels.iter()) {
            println!("{},{},{}", point.0, point.1, label);
        }
       
    }

    pub fn predict_class(&mut self) {
        unimplemented!();
    }

    fn calc_center(&mut self) {
        let centers: Vec<(f64, f64)> = Vec::new();
        let mut sum: Vec<(f64, f64)> = Vec::new();

        for i in 0..self.n_clusters {
            sum.push((0f64, 0f64));
        }

        for (point, label) in self.points.iter().zip(self.labels.iter()) {
            sum[*label as usize].0 += point.0;
            sum[*label as usize].1 += point.1;
        }

        for (val, count) in sum.iter_mut().zip(self.labels_count.iter()) {
            val.0 /= (*count) as f64;
            val.1 /= (*count) as f64;
        }

        self.cluster_centers = sum;
    }

    fn calc_next_label(&mut self) -> bool {
        let mut result = true;
        for (idx, point) in self.points.iter().enumerate() {
            let mut min_distance = 
            (
                (point.0 - self.cluster_centers[0].0).abs() * (point.0 - self.cluster_centers[0].0).abs() +
                (point.1 - self.cluster_centers[0].1).abs() * (point.1 - self.cluster_centers[0].1).abs()
            ).sqrt();
            let mut label = 0;
            for i in 1..self.n_clusters {
                let distance = 
                (
                    (point.0 - self.cluster_centers[i as usize].0).abs() * (point.0 - self.cluster_centers[i as usize].0).abs() +
                    (point.1 - self.cluster_centers[i as usize].1).abs() * (point.1 - self.cluster_centers[i as usize].1).abs()
                ).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    label = i;
                }
            }
            if self.labels[idx] != label {
                self.labels_count[self.labels[idx] as usize] -= 1;
                self.labels_count[label as usize] += 1;
                self.labels[idx] = label;
                result = false;
            }
        }

        result
    }

    fn calc_distance(&mut self) {
        
    } 
}
