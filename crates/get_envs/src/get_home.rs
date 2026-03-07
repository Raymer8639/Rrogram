use std::process;
use tracing::debug;
use tracing::error;

pub fn get_home() -> String {
    let result = home::home_dir()
        .unwrap_or_else(|| {
            error!("Cannot get home directory");
            process::exit(1);
        })
        .display()
        .to_string();
    debug!("Home: {result}");
    result
}
