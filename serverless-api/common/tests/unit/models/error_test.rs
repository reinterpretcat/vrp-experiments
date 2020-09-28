use super::*;

#[test]
fn can_format_app_error() {
    let error = AppError {
        message: "some msg".to_string(),
        details: "some detail".to_string(),
    };

    let result = format!("{}", error);

    assert!(result.contains("{"));
    assert!(result.contains("message"));
    assert!(result.contains("details"));
}
