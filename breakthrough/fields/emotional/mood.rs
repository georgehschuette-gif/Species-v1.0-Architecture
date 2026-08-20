// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// EmotionalMood: Sustained affective disposition over time.
pub struct EmotionalMood {
    pub baseline_valence: EmotionalValence,
    pub baseline_arousal: EmotionalArousal,
    pub stability: f64,
    pub decay_rate: f64,
    pub current_valence: EmotionalValence,
    pub current_arousal: EmotionalArousal,
}

impl EmotionalMood {
    pub fn new(baseline_valence: EmotionalValence, baseline_rousal: EmotionalArousal, stability: f64, decay_rate: f64) -> Result<Self, EmotionalError> {
        if !(0.0..=1.0).contains(&stability) {
            return Err(EmotionalError::InvalidStability { stability });
        }
        if !(0.0..=1.0).contains(&decay_rate) {
            return Err(EmotionalError::InvalidDecay { decay: decay_rate });
        }
        Ok(Self {
            baseline_valence,
            baseline_arousal: baseline_rousal,
            stability,
            decay_rate,
            current_valence: baseline_valence,
            current_arousal: baseline_rousal,
        })
    }

    pub fn update(&mut self, stimulus_valence: EmotionalValence, stimulus_arousal: EmotionalArousal, weight: f64) {
        let w = weight.clamp(0.0, 1.0);
        self.current_valence = self.current_valence.blend(stimulus_valence, w * (1.0 - self.stability));
        self.current_arousal = EmotionalArousal(
            (self.current_arousal.0 * (1.0 - self.decay_rate) + stimulus_arousal.0 * self.decay_rate * w).clamp(EmotionalArousal::MIN, EmotionalArousal::MAX)
        );
    }

    pub fn drift_toward_baseline(&mut self) {
        self.current_valence = self.current_valence.blend(self.baseline_valence, self.stability);
        self.current_arousal = EmotionalArousal(
            (self.current_arousal.0 * (1.0 - self.stability) + self.baseline_arousal.0 * self.stability).clamp(EmotionalArousal::MIN, EmotionalArousal::MAX)
        );
    }
}
