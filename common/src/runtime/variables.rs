pub fn get_problem_key(submission_id: &str) -> String {
    format!("{}/problem.json", submission_id)
}

pub fn get_state_key(submission_id: &str) -> String {
    format!("{}/state.json", submission_id)
}
