// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ClusteringDetector: Identifies anomalies via outlier clustering.
pub struct ClusteringDetector {
    pub cluster_count: usize,
    pub outlier_threshold: f64,
    pub clusters: Vec<Cluster>,
    pub anomalies: Vec<Vec<f64>>,
}

/// A cluster of similar data points.
#[derive(Debug, Clone)]
pub struct Cluster {
    pub id: usize,
    pub centroid: Vec<f64>,
    pub members: Vec<Vec<f64>>,
    pub radius: f64,
}

impl ClusteringDetector {
    /// Create a new clustering detector.
    pub fn new(cluster_count: usize, outlier_threshold: f64) -> PerceptionResult<Self> {
        if cluster_count == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "cluster_count must be positive".into(),
            ));
        }
        if outlier_threshold <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "outlier_threshold must be positive".into(),
            ));
        }
        Ok(Self {
            cluster_count,
            outlier_threshold,
            clusters: Vec::new(),
            anomalies: Vec::new(),
        })
    }

    /// Assign a data point to the nearest cluster or create a new one.
    pub fn assign(&mut self, point: &[f64]) -> PerceptionResult<usize> {
        if point.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        if self.clusters.len() < self.cluster_count {
            let id = self.clusters.len();
            self.clusters.push(Cluster {
                id,
                centroid: point.to_vec(),
                members: vec![point.to_vec()],
                radius: 0.0,
            });
            return Ok(id);
        }
        let nearest = self.find_nearest_cluster(point)?;
        self.clusters[nearest].members.push(point.to_vec());
        self.update_centroid(nearest);
        self.update_radius(nearest);
        Ok(nearest)
    }

    /// Find the index of the nearest cluster to a point.
    pub fn find_nearest_cluster(&self, point: &[f64]) -> PerceptionResult<usize> {
        if self.clusters.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let mut nearest = 0;
        let mut min_dist = f64::MAX;
        for (i, cluster) in self.clusters.iter().enumerate() {
            if cluster.centroid.len() != point.len() {
                continue;
            }
            let dist = euclidean_distance(&cluster.centroid, point);
            if dist < min_dist {
                min_dist = dist;
                nearest = i;
            }
        }
        Ok(nearest)
    }

    /// Update a cluster's centroid from its members.
    fn update_centroid(&mut self, idx: usize) {
        let cluster = &self.clusters[idx];
        if cluster.members.is_empty() {
            return;
        }
        let dim = cluster.members[0].len();
        let mut centroid = vec![0.0; dim];
        for member in &cluster.members {
            for (j, &v) in member.iter().enumerate() {
                centroid[j] += v;
            }
        }
        for v in &mut centroid {
            *v /= cluster.members.len() as f64;
        }
        self.clusters[idx].centroid = centroid;
    }

    /// Update a cluster's radius based on member distances.
    fn update_radius(&mut self, idx: usize) {
        let cluster = &self.clusters[idx];
        if cluster.members.is_empty() {
            return;
        }
        let mut max_dist = 0.0;
        for member in &cluster.members {
            let dist = euclidean_distance(&cluster.centroid, member);
            if dist > max_dist {
                max_dist = dist;
            }
        }
        self.clusters[idx].radius = max_dist;
    }

    /// Detect anomalies in a data point based on distance from nearest cluster centroid.
    pub fn detect_anomaly(&mut self, point: &[f64]) -> PerceptionResult<bool> {
        if self.clusters.is_empty() {
            self.assign(point)?;
            return Ok(false);
        }
        let nearest_idx = self.find_nearest_cluster(point)?;
        let dist = euclidean_distance(&self.clusters[nearest_idx].centroid, point);
        let is_anomaly = dist > self.outlier_threshold;
        if is_anomaly {
            self.anomalies.push(point.to_vec());
        }
        Ok(is_anomaly)
    }

    /// Return the cluster containing the most members.
    pub fn largest_cluster(&self) -> PerceptionResult<Option<&Cluster>> {
        self.clusters
            .iter()
            .max_by(|a, b| a.members.len().cmp(&b.members.len()))
            .map(Some)
            .ok_or_else(|| PerceptionError::InsufficientData)
    }

    /// Return the number of detected anomalies.
    pub fn anomaly_count(&self) -> usize {
        self.anomalies.len()
    }

    /// Return the number of formed clusters.
    pub fn formed_clusters(&self) -> usize {
        self.clusters.len()
    }

    /// Set the number of clusters.
    pub fn set_cluster_count(&mut self, count: usize) -> PerceptionResult<()> {
        if count == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "cluster_count must be positive".into(),
            ));
        }
        self.cluster_count = count;
        if self.clusters.len() > count {
            self.clusters.truncate(count);
        }
        Ok(())
    }

    /// Set the outlier threshold.
    pub fn set_outlier_threshold(&mut self, threshold: f64) -> PerceptionResult<()> {
        if threshold <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "outlier_threshold must be positive".into(),
            ));
        }
        self.outlier_threshold = threshold;
        Ok(())
    }

    /// Reset clusters and anomaly records.
    pub fn reset(&mut self) {
        self.clusters.clear();
        self.anomalies.clear();
    }

    /// Validate the detector configuration.
    pub fn validate(&self) -> PerceptionResult<()> {
        if self.cluster_count == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "cluster_count must be positive".into(),
            ));
        }
        if self.outlier_threshold <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "outlier_threshold must be positive".into(),
            ));
        }
        Ok(())
    }
}

fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return f64::MAX;
    }
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}