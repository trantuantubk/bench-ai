use bench_ai::models::Model;

#[test]
fn test_valid_model_creation() {
    // Arrange
    let name = "resnet50".to_string();
    let framework = "onnx".to_string();
    let version = "1.0".to_string();
    
    // Act
    let model = Model::new(name.clone(), framework.clone(), version.clone())
        .expect("valid model");

    // Assert
    assert_eq!(model.name(), &name);
    assert_eq!(model.framework(), &framework);
    assert_eq!(model.version(), &version);
}

#[test]
fn test_model_validation_empty_name() {
    // Act
    let result = Model::new("".to_string(), "onnx".to_string(), "1.0".to_string());

    // Assert
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("name cannot be empty"));

}

#[test]
fn test_model_validation_invalid_framework() {
    // Act
    let result = Model::new(
        "resnet50".to_string(),
        "invalid".to_string(),
        "1.0".to_string()
    );

    // Assert
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unsupported framework"));

}