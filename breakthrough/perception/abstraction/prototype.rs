// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// PrototypeFormation: Creates central representations from category members.
pub struct PrototypeFormation {
    pub update_rate: f64,
    pub prototypes: Vec<Prototype>,
}

/// A central representative vector for a category of percepts.
#[derive(Debug, Clone)]
pub struct Prototype {
    pub id: usize,
    pub features: Vec<f64>,
    pub quality: f64,
}

impl PrototypeFormation {
    /// Create a new prototype formation module.
    pub fn new(update_rate: f64) -> PerceptionResult<Self> {
        if !(0.0..=1.0).contains(&update_rate) {
            return Err(PerceptionError::InvalidConfiguration(
                "update_rate must be in [0, 1]".into(),
            ));
        }
        Ok(Self {
            update_rate,
            prototypes: Vec::new(),
        })
    }

    /// Create or update a prototype from a set of member feature vectors.
    pub fn form_prototype(
        &mut self,
        members: &[Vec<f64>],
    ) -> PerceptionResult<usize> {
        if members.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let dim = members[0].len();
        if members.iter().any(|m| m.len() != dim) {
            return Err(PerceptionError::InvalidConfiguration(
                "all member vectors must have the same dimension".into(),
            ));
        }
        let mut centroid = vec![0.0; dim];
        for member in members {
            for (j, &v) in member.iter().enumerate() {
                centroid[j] += v;
            }
        }
        for v in &mut centroid {
            *v /= members.len() as f64;
        }
        let quality = self.compute_quality(&centroid, members);
        let id = self.prototypes.len();
        self.prototypes.push(Prototype {
            id,
            features: centroid,
            quality,
        });
        Ok(id)
    }

    /// Update an existing prototype toward new member data.
    pub fn update_prototype(
        &mut self,
        prototype_id: usize,
        members: &[Vec<f64>],
    ) -> PerceptionResult<()> {
        if members.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let dim = self
            .prototypes
            .get(prototype_id)
            .ok_or_else(|| {
                PerceptionError::InvalidConfiguration(
                    format!("prototype {} not found", prototype_id),
                )
            })?
            .features
            .len();
        let mut centroid = self
            .prototypes
            .get(prototype_id)
            .ok_or_else(|| {
                PerceptionError::InvalidConfiguration(
                    format!("prototype {} not found", prototype_id),
                )
            })?
            .features
            .clone();
        let alpha = self.update_rate;
        for member in members {
            if member.len() != dim {
                return Err(PerceptionError::InvalidConfiguration(
                    "member dimension mismatch".into(),
                ));
            }
            for i in 0..dim {
                centroid[i] = (1.0 - alpha) * centroid[i] + alpha * member[i];
            }
        }
        let quality = self.compute_quality(&centroid, members);
        let proto = self
            .prototypes
            .get_mut(prototype_id)
            .ok_or_else(|| {
                PerceptionError::InvalidConfiguration(
                    format!("prototype {} not found", prototype_id),
                )
            })?;
        proto.features = centroid;
        proto.quality = quality;
        Ok(())
    }

    /// Compute the quality of a prototype as the average inverse distance to its members.
    fn compute_quality(&self, centroid: &[f64], members: &[Vec<f64>]) -> f64 {
        if members.is_empty() {
            return 0.0;
        }
        let mut total_dist = 0.0;
        for member in members {
            let dist = euclidean_distance(centroid, member);
            total_dist += dist;
        }
        let avg_dist = total_dist / members.len() as f64;
        if avg_dist == 0.0 {
            1.0
        } else {
            1.0 / (1.0 + avg_dist)
        }
    }

    /// Return the best prototype for a given feature vector.
    pub fn best_match(&self, features: &[f64]) -> PerceptionResult<Option<usize>> {
        if self.prototypes.is_empty() {
            return Ok(None);
        }
        let mut best_idx = 0;
        let mut best_dist = f64::MAX;
        for (i, proto) in self.prototypes.iter().enumerate() {
            if proto.features.len() != features.len() {
                continue;
            }
            let dist = euclidean_distance(&proto.features, features);
            if dist < best_dist {
                best_dist = dist;
                best_idx = i;
            }
        }
        Ok(Some(best_idx))
    }

    /// Return the number of prototypes formed.
    pub fn prototype_count(&self) -> usize {
        self.prototypes.len()
    }

    /// Get a reference to a prototype by index.
    pub fn get_prototype(&self, idx: usize) -> Option<&Prototype> {
        self.prototypes.get(idx)
    }

    /// Validate all prototype data is consistent.
    pub fn validate(&self) -> PerceptionResult<()> {
        for (i, proto) in self.prototypes.iter().enumerate() {
            if proto.features.is_empty() {
                return Err(PerceptionError::InvalidConfiguration(
                    format!("prototype {} has empty feature vector", i),
                ));
            }
            if !(0.0..=1.0).contains(&proto.quality) {
                return Err(PerceptionError::InvalidConfiguration(
                    format!("prototype {} quality out of range", i),
                ));
            }
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