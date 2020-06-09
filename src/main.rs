use lambda_http::{lambda, IntoResponse, Request, RequestExt, Response};
use serde::Deserialize;

#[derive(Deserialize)]
struct Problem {
    pub id: String,
    pub plan: Vec<String>,
    pub fleet: Vec<String>,
}

fn main() {
    lambda!(|request: Request, _context| {
        let problem: Option<Problem> = request.payload().unwrap_or_else(|_parse_err| None);

        let response = if let Some(problem) = problem {
            format!("problem id: {}", problem.id).into_response()
        } else {
            Response::builder()
                .status(400)
                .body("Empty first name".into())
                .expect("failed to render response")
        };

        Ok(response)
    })
}
