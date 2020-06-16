use super::*;

#[test]
fn can_create_created_response() {
    let result = created(None);

    assert_eq!(result.status_code, 201);
}