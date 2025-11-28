// PRE-MAN, a cli tool to manage api calls

mod structs;
mod utils;
mod request;
mod cli;

use request::Request;

#[tokio::main]
async fn main() {
	let args = cli::Args::parse_args();

	let logger = utils::Logger::new();

	let mut request = Request::new(&args.method, &args.formatted_url());
	let response = request.send().await;

	match response {
		Ok(_) => {
			let resp = request.response.as_ref().unwrap();
			logger.log_response(resp, args.show_option());
		}
		Err(e) => {
			logger.error(&format!("Request failed: {}", e));
			std::process::exit(1);
		}
	}
}
