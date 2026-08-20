// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// CategoryFormation: Groups percepts into abstract categories.
pub struct CategoryFormation {
    pub min_members: usize,
    pub similarity_threshold: f64,
    pub categories: Vec<Category>,
}

/// A group of related percepts sharing common features.
#[derive(Debug, Clone)]
pub struct Category {
    pub id: usize,
    pub members: Vec<Vec<f64>>,
    pub centroid: Vec<f64>,
    pub label: String,
}

impl CategoryFormation {
    /// Create a new category formation module.
    pub fn new(min_members: usize, similarity_threshold: f64) -> PerceptionResult<Self> {
        if min_members == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "min_members must be positive".into(),
            ));
        }
        if !(0.0..=1.0).contains(&similarity_threshold) {
            return Err(PerceptionError::InvalidConfiguration(
                "similarity_threshold must be in [0, 1]".into(),
            ));
        }
        Ok(Self {
            min_members,
            similarity_threshold,
            categories: Vec::new(),
        })
    }

    /// Add a feature vector to the nearest matching category or create a new one.
    pub fn add_percept(&mut self, features: Vec<f64>) -> PerceptionResult<usize> {
        if features.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        if let Some((cat_idx, sim)) = self.find_best_category(&features)? {
            if sim >= self.similarity_threshold {
                self.categories[cat_idx].members.push(features.clone());
                self.recompute_centroid(cat_idx);
                return Ok(self.categories[cat_idx].id);
            }
        }
        let id = self.categories.len();
        let category = Category {
            id,
            members: vec![features.clone()],
            centroid: features.clone(),
            label: format!("category_{}", id),
        };
        self.categories.push(category);
        Ok(id)
    }

    /// Find the best matching category for a feature vector.
    pub fn find_best_category(
        &self,
        features: &[f64],
    ) -> PerceptionResult<Option<(usize, f64)>> {
        if self.categories.is_empty() {
            return Ok(None);
        }
        let mut best_idx = 0;
        let mut best_sim = f64::NEG_INFINITY;
        for (i, cat) in self.categories.iter().enumerate() {
            if cat.centroid.len() != features.len() {
                continue;
            }
            let sim = cosine_similarity(&cat.centroid, features)?;
            if sim > best_sim {
                best_sim = sim;
                best_idx = i;
            }
        }
        Ok(Some((best_idx, best_sim)))
    }

    /// Recompute the centroid of a category from its members.
    fn recompute_centroid(&mut self, idx: usize) {
        if idx >= self.categories.len() || self.categories[idx].members.is_empty() {
            return;
        }
        let member_count = self.categories[idx].members.len();
        let dim = self.categories[idx].members[0].len();
        let mut centroid = vec![0.0; dim];
        for member in &self.categories[idx].members {
            for (j, &v) in member.iter().enumerate() {
                centroid[j] += v;
            }
        }
        for v in &mut centroid {
            *v /= member_count as f64;
        }
        self.categories[idx].centroid = centroid;
    }

    /// Return the number of categories formed.
    pub fn category_count(&self) -> usize {
        self.categories.len()
    }

    /// Return the number of percepts across all categories.
    pub fn total_members(&self) -> usize {
        self.categories.iter().map(|c| c.members.len()).sum()
    }

    /// Remove categories with fewer than min_members.
    pub fn prune_small_categories(&mut self) -> usize {
        let before = self.categories.len();
        self.categories.retain(|c| c.members.len() >= self.min_members);
        before - self.categories.len()
    }

    /// Get a reference to a category by index.
    pub fn get_category(&self, idx: usize) -> Option<&Category> {
        self.categories.get(idx)
    }

    /// Validates the category formation state.
    pub fn validate(&self) -> PerceptionResult<()> {
        if self.min_members == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "min_members must be positive".into(),
            ));
        }
        if !(0.0..=1.0).contains(&self.similarity_threshold) {
            return Err(PerceptionError::InvalidConfiguration(
                "similarity_threshold must be in [0, 1]".into(),
            ));
        }
        Ok(())
    }
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> PerceptionResult<f64> {
    if a.len() != b.len() {
        return Err(PerceptionError::InvalidConfiguration(
            "feature dimension mismatch".into(),
        ));
    }
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|v| v * v).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|v| v * v).sum::<f64>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return Ok(0.0);
    }
    Ok(dot / (norm_a * norm_b))
}