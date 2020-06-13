use aws_lambda_events::event::apigw::ApiGatewayProxyResponse as Response;

fn default_response() -> Response {
    Response {
        status_code: 0,
        is_base64_encoded: Some(false),
        multi_value_headers: Default::default(),
        headers: Default::default(),
        body: None,
    }
}

pub fn created(body: Option<String>) -> Response {
    Response {
        status_code: 201,
        body,
        ..default_response()
    }
}

pub fn bad_request(body: Option<String>) -> Response {
    Response {
        status_code: 400,
        body,
        ..default_response()
    }
}

pub fn internal_server_error(body: Option<String>) -> Response {
    Response {
        status_code: 500,
        body,
        ..default_response()
    }
}
