extern crate log;
extern crate env_logger;
extern crate dotenv;

use log::{debug, error, info};

use dotenv::dotenv;

fn main() {
	init_env();
	init_log();
}

fn init_log() {
	env_logger::init();
	info! {"logger initialised"};
}

fn init_env() {
	dotenv().ok();
}

