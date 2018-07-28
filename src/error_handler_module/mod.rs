use rocket::Request;

#[catch(500)]
fn internal_error() -> &'static str {
  "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
  format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(400)]
fn unmatch_request(req: &Request) -> String {
  format!(
    "JSON format does not match or missing field at '{}'",
    req.uri()
  )
}
