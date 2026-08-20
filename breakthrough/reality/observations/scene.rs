// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use super::{ObservationConfidence, SensoryModality};
use crate::RealityError;

/// ObjectDetection: A detected object within an observation.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectDetection {
    /// Unique identifier for this detected object.
    pub id: String,
    /// Category or class label.
    pub category: String,
    /// Bounding box or region specification.
    pub region: Region,
    /// Confidence in this detection.
    pub confidence: ObservationConfidence,
    /// Additional attributes of the detected object.
    pub attributes: HashMap<String, String>,
}

impl ObjectDetection {
    /// Creates a new object detection.
    pub fn new(
        id: impl Into<String>,
        category: impl Into<String>,
        region: Region,
        confidence: ObservationConfidence,
    ) -> Self {
        Self {
            id: id.into(),
            category: category.into(),
            region,
            confidence,
            attributes: HashMap::new(),
        }
    }

    /// Adds an attribute to the detection.
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Returns the overlap between this detection's region and another's.
    pub fn overlap(&self, other: &ObjectDetection) -> f64 {
        self.region.intersection_area(&other.region)
            / self.region.union_area(&other.region).max(f64::EPSILON)
    }

    /// Determines whether this detection is sufficiently similar to another.
    pub fn is_similar(&self, other: &ObjectDetection, threshold: f64) -> bool {
        self.category == other.category && self.overlap(other) >= threshold
    }
}

/// Region: A spatial or abstract region specification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Region {
    /// Minimum x coordinate or start index.
    pub min_x: f64,
    /// Minimum y coordinate or start index.
    pub min_y: f64,
    /// Maximum x coordinate or end index.
    pub max_x: f64,
    /// Maximum y coordinate or end index.
    pub max_y: f64,
}

impl Region {
    /// Creates a new region from bounds.
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self { min_x, min_y, max_x, max_y }
    }

    /// Returns the width of the region.
    pub fn width(&self) -> f64 {
        (self.max_x - self.min_x).max(0.0)
    }

    /// Returns the height of the region.
    pub fn height(&self) -> f64 {
        (self.max_y - self.min_y).max(0.0)
    }

    /// Returns the area of the region.
    pub fn area(&self) -> f64 {
        self.width() * self.height()
    }

    /// Computes the intersection area with another region.
    pub fn intersection_area(&self, other: &Region) -> f64 {
        let x_overlap = (self.max_x.min(other.max_x) - self.min_x.max(other.min_x)).max(0.0);
        let y_overlap = (self.max_y.min(other.max_y) - self.min_y.max(other.min_y)).max(0.0);
        x_overlap * y_overlap
    }

    /// Computes the union area with another region.
    pub fn union_area(&self, other: &Region) -> f64 {
        self.area() + other.area() - self.intersection_area(other)
    }

    /// Returns whether this region contains a point.
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }
}

/// Scene: A structured representation of the observed environment.
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    /// Unique identifier for this scene observation.
    pub id: String,
    /// Timestamp of the observation.
    pub timestamp: f64,
    /// Detected objects within the scene.
    pub objects: Vec<ObjectDetection>,
    /// Global scene attributes.
    pub attributes: HashMap<String, String>,
    /// Overall confidence in the scene model.
    pub confidence: ObservationConfidence,
    /// The sensory modalities contributing to this scene.
    pub modalities: Vec<SensoryModality>,
}

impl Scene {
    /// Creates a new empty scene.
    pub fn new(id: impl Into<String>, timestamp: f64) -> Self {
        Self {
            id: id.into(),
            timestamp,
            objects: Vec::new(),
            attributes: HashMap::new(),
            confidence: ObservationConfidence::default(),
            modalities: Vec::new(),
        }
    }

    /// Adds an object detection to the scene.
    pub fn add_object(&mut self, detection: ObjectDetection) {
        self.objects.push(detection);
    }

    /// Adds a global attribute to the scene.
    pub fn add_attribute(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attributes.insert(key.into(), value.into());
    }

    /// Adds a sensory modality to the scene.
    pub fn add_modality(&mut self, modality: SensoryModality) {
        if !self.modalities.contains(&modality) {
            self.modalities.push(modality);
        }
    }

    /// Returns the number of objects in the scene.
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Returns objects matching a category.
    pub fn objects_by_category(&self, category: &str) -> Vec<&ObjectDetection> {
        self.objects.iter().filter(|o| o.category == category).collect()
    }

    /// Returns objects with confidence above a threshold.
    pub fn high_confidence_objects(&self, threshold: f64) -> Vec<&ObjectDetection> {
        self.objects.iter().filter(|o| o.confidence.0 >= threshold).collect()
    }

    /// Computes the average confidence across all objects.
    pub fn average_confidence(&self) -> ObservationConfidence {
        if self.objects.is_empty() {
            return ObservationConfidence::default();
        }
        let sum: f64 = self.objects.iter().map(|o| o.confidence.0).sum();
        ObservationConfidence::new(sum / self.objects.len() as f64)
            .unwrap_or(ObservationConfidence::default())
    }
}
