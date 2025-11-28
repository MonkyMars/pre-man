use crate::structs::Response;

pub struct Logger {
		
}

impl Logger {
		pub fn new() -> Self {
				Logger {
						
				}
		}

		pub fn info(&self, message: &str) {
				println!("INFO: {}", message);
		}

		pub fn error(&self, message: &str) {
				eprintln!("ERROR: {}", message);
		}

		pub fn log_response(&self, response: &Response, show: Vec<&str>) {
			if show.is_empty() {
				self.info(&format!("Status: {}", response.status));
				self.info(&format!("Duration: {} ms", response.duration.num_milliseconds()));
				self.info(&format!("Body: {}", response.body));
				self.info(&format!("Success: {}", response.success));
			} else {
				for field in show {
					match field {
						"status" => self.info(&format!("Status: {}", response.status)),
						"duration" => self.info(&format!("Duration: {} ms", response.duration.num_milliseconds())),
						"body" => self.info(&format!("Body: {}", response.body)),
						"success" => self.info(&format!("Success: {}", response.success)),
						_ => {}
					}
				}
			}
		}
}