/// Change this route handler to return a custom error status code of 588 and a JSON object that looks like
///
/// ```json
/// {
///    "code": 1,
///    "message": "There was an error in the server"
/// }
/// ```
pub async fn non_standard_error_with_json() {}
