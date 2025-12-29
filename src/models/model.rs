use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Model {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    name: String,
    framework: String,
    version: String,
}

#[derive(Debug)]
pub struct ModelError {
    message: String,
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ModelError {}

impl Model {
    /// Creates a new Model with validation
    pub fn new(name: String, framework: String, version: String) -> Result<Self, String> {
        // Validate name
        if name.trim().is_empty() {
            return Err("Model name cannot be empty".to_string());
        }

        // Validate framework
        let valid_frameworks = vec!["onnx", "tensorflow", "pytorch", "tflite"];
        if !valid_frameworks.contains(&framework.to_lowercase().as_str()) {
            return Err(format!(
                "Unsupported framework: {}. Supported: {:?}",
                framework, valid_frameworks
            ));
        }

        // Validate version
        if version.trim().is_empty() {
            return Err("Model version cannot be empty".to_string());
        }

        Ok(Model {
            id: Uuid::new_v4(),
            name: name.trim().to_string(),
            framework: framework.to_lowercase(),
            version: version.trim().to_string(),
        })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn framework(&self) -> &str {
        &self.framework
    }

    pub fn version(&self) -> &str {
        &self.version
    }

}
