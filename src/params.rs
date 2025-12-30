//! Parameter definitions for synths and effects

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Parameter definition with name, default value, and range
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ParameterDef {
    pub name: String,
    pub default: f32,
    pub min: f32,
    pub max: f32,
}

impl ParameterDef {
    /// Create a new parameter definition
    pub fn new(name: impl Into<String>, default: f32, min: f32, max: f32) -> Self {
        Self {
            name: name.into(),
            default,
            min,
            max,
        }
    }

    /// Clamp a value to this parameter's range
    pub fn clamp(&self, value: f32) -> f32 {
        value.clamp(self.min, self.max)
    }

    /// Normalize a value to 0.0-1.0 range
    pub fn normalize(&self, value: f32) -> f32 {
        if self.max == self.min {
            0.0
        } else {
            (value - self.min) / (self.max - self.min)
        }
    }

    /// Denormalize from 0.0-1.0 to actual range
    pub fn denormalize(&self, normalized: f32) -> f32 {
        self.min + normalized * (self.max - self.min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_def() {
        let param = ParameterDef::new("frequency", 440.0, 20.0, 20000.0);
        assert_eq!(param.name, "frequency");
        assert_eq!(param.default, 440.0);
        assert_eq!(param.clamp(10.0), 20.0);
        assert_eq!(param.clamp(30000.0), 20000.0);
    }

    #[test]
    fn test_normalize() {
        let param = ParameterDef::new("volume", 0.5, 0.0, 1.0);
        assert_eq!(param.normalize(0.5), 0.5);
        assert_eq!(param.normalize(0.0), 0.0);
        assert_eq!(param.normalize(1.0), 1.0);
    }

    #[test]
    fn test_denormalize() {
        let param = ParameterDef::new("frequency", 440.0, 100.0, 1000.0);
        assert_eq!(param.denormalize(0.0), 100.0);
        assert_eq!(param.denormalize(1.0), 1000.0);
        assert_eq!(param.denormalize(0.5), 550.0);
    }
}
