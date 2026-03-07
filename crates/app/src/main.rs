use get_envs::get_rrogram_home;
use menu::{get_content, main_menu};
use starter::{continue_project, manage_project, new_project};
use std::env;
use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    info!("Checking for envs ...");
    let rrogram_home = get_rrogram_home::get_rrogram();
    debug!("Rrogram_home: {rrogram_home}");
    info!("Checking for envs ... ok");
    debug!("App started");
    println!("Rrogram v{}", VERSION);
    main_menu::show();
    loop {
        let user_input: String = get_content::get(None).expect("?");
        let user_input = user_input.as_str();

        match user_input {
            "0" => continue_project::continue_project(rrogram_home.clone()),
            "1" => new_project::new_project(rrogram_home.clone()),
            "2" => manage_project::manage_project(rrogram_home.clone()),
            "3" => break,
            _ => println!("?"),
        }
    }
}
