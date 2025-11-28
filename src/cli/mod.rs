use clap::Parser;
use crate::utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
		pub method: String,
		#[arg(short, long)]
		pub url: String,
		#[arg(short, long)]
		pub show: Option<String>,
}

impl Args {
		pub fn parse_args() -> Self {
				Args::parse()
		}

		pub fn formatted_url(&self) -> String {
				utils::format_url(&self.url)
		}

		pub fn show_option(&self) -> Vec<&str> {
				if self.show.is_none() {
						return vec![];
				}
				self.show.as_ref().unwrap().split(',').collect()
		}
}