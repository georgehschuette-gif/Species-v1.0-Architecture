// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ChainReaction {
    pub steps: Vec<String>,
    pub current_step: usize,
    pub is_running: bool,
}

impl ChainReaction {
    pub fn new(initial_step: String) -> Self {
        Self { steps: vec![initial_step], current_step: 0, is_running: true }
    }

    pub fn add_step(&mut self, step: String) {
        self.steps.push(step);
    }

    pub fn advance(&mut self) -> Option<&str> {
        if !self.is_running || self.current_step >= self.steps.len() - 1 {
            self.is_running = false;
            return None;
        }
        self.current_step += 1;
        Some(&self.steps[self.current_step])
    }

    pub fn current(&self) -> Option<&str> {
        self.steps.get(self.current_step).map(|s| s.as_str())
    }
}

impl fmt::Display for ChainReaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChainReaction(step={}/{}, running={})", self.current_step + 1, self.steps.len(), self.is_running)
    }
}

