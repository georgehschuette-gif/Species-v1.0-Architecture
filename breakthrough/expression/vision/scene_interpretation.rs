// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct SceneInterpretation {
    pub image_id: String,
    pub labels: Vec<String>,
    pub confidence: f64,
    pub bounding_boxes: Vec<BoundingBox>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl SceneInterpretation {
    pub fn new<S: Into<String>>(image_id: S) -> Self {
        Self {
            image_id: image_id.into(),
            labels: Vec::new(),
            confidence: 0.0,
            bounding_boxes: Vec::new(),
            description: String::new(),
        }
    }

    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn add_bounding_box(&mut self, bbox: BoundingBox) {
        self.bounding_boxes.push(bbox);
    }

    pub fn object_count(&self) -> usize {
        self.bounding_boxes.len()
    }

    pub fn is_high_confidence(&self, threshold: f64) -> bool {
        self.confidence >= threshold
    }
}

impl BoundingBox {
    pub fn new<S: Into<String>>(x: u32, y: u32, width: u32, height: u32, label: S) -> Self {
        Self {
            x,
            y,
            width,
            height,
            label: label.into(),
        }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl fmt::Display for SceneInterpretation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scene {} ({} objects, {:.2} confidence)", self.image_id, self.object_count(), self.confidence)
    }
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at ({},{}) {}x{}", self.label, self.x, self.y, self.width, self.height)
    }
}

