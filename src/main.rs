extern crate dotenv;
extern crate env_logger;
extern crate log;
extern crate failure;
extern crate r2d2_sqlite;

use r2d2_sqlite::SqliteConnectionManager;

use failure::{bail, Error};

use log::{debug, error, info};

use std::env;
use std::fs::File;
use std::path::Path;

use dotenv::dotenv;


// CONSTANTS ++++++++++++++++++++++++++++
pub const PARDAL_DB_PATH_ENV_NAME: &str = "PARDAL_DB_PATH";
// ++++++++++++++++++++++++++++++++++++++


fn main() {
	
	init_env();
	init_log();
	init_db();
	
}

fn init_log() {
	env_logger::init();
	debug! {"logger initialised"};
}

fn init_env() {
	dotenv().ok();
}

fn init_db() -> Result<SqliteConnectionManager, Error> {
	
	let db_path = env::var(PARDAL_DB_PATH_ENV_NAME).unwrap_or_else(|_| String::from("pardal.db"));
	let db_path = Path::new(&db_path);

	let db_file = File::open(&db_path);

	if let Err(error) = db_file {
		debug!(
				"Open sqlite db file at: {}, value retrieved from \"{}\" env, failed with cause \"{}\", trying to create new file ... ",
				&db_path.display(),
				PARDAL_DB_PATH_ENV_NAME,
				error
				);

		let db_file = File::create(&db_path);

		if let Err(error) = db_file {
			error! {"Create sqlite db file at {} failed. Cause: {}",
			 &db_path.display(),
			error};
			bail!(error);
		}
	}

	let conn_manager = SqliteConnectionManager::file(&db_path);
	info! {"db initialised at: {}", &db_path.canonicalize()?.as_path().display()};

	Ok(conn_manager)
}
