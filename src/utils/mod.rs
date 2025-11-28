mod timer;
mod log;

pub use timer::Timer;
pub use log::Logger;

pub fn format_url(url: &str) -> String {
		if url.starts_with("http://") || url.starts_with("https://") {
				url.to_string()
		} else {
				format!("http://{}", url)
		}
}