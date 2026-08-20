// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionType {
    Physical,
    Digital,
    Communication,
    Cognitive,
    Composite,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Action {
    pub id: String,
    pub description: String,
    pub action_type: ActionType,
    pub parameters: Vec<(String, String)>,
    pub preconditions: Vec<String>,
    pub effects: Vec<String>,
}

impl Action {
    pub fn new<S: Into<String>>(id: S, description: S, action_type: ActionType) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            action_type,
            parameters: Vec::new(),
            preconditions: Vec::new(),
            effects: Vec::new(),
        }
    }

    pub fn add_parameter<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.parameters.push((key.into(), value.into()));
    }

    pub fn add_precondition<S: Into<String>>(&mut self, condition: S) {
        self.preconditions.push(condition.into());
    }

    pub fn add_effect<S: Into<String>>(&mut self, effect: S) {
        self.effects.push(effect.into());
    }

    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self.action_type, ActionType::Physical | ActionType::Digital)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action {}: {} ({:?})", self.id, self.description, self.action_type)
    }
}

