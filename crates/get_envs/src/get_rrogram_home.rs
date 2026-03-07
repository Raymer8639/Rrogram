use crate::get_home;
use std::env;
use tracing::{error, warn};

pub fn get_rrogram() -> String {
    match env::var("RROGRAM_HOME") {
        Ok(r) => r,
        Err(_) => {
            error!("Cannot get `$RROGRAM_HOME`");
            warn!("Cannot get `$RROGRAM_HOME` -- Using the default");
            get_home::get_home() + "/.rrogram"
        }
    }
}
