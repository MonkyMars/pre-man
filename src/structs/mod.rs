#[derive(Debug, Clone)]
pub struct Response {
	pub body: String,
	pub status: httpstatus::StatusCode,
	pub duration: chrono::Duration,
	pub success: bool,
}