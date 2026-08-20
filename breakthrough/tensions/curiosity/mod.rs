// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::TensionsError;

pub mod drive;
pub mod saturation;
pub mod reward;

pub use drive::CuriosityDrive;
pub use saturation::CuriositySaturation;
pub use reward::CuriosityReward;

pub const DEFAULT_CURIOUSITY_DRIVE: f64 = 0.7;
pub const MAX_CURIOUSITY: f64 = 1.0;

pub fn create_curiosity_drive(intensity: f64) -> Result<CuriosityDrive, TensionsError> {
    CuriosityDrive::new(intensity)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn curiosity_drive_creation() {
        let d = create_curiosity_drive(0.8).unwrap();
        assert!(d.intensity() > 0.0);
    }
}
