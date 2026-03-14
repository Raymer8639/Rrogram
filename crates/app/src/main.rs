use args::{Args, This};
use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use get_envs::get_home;
use menu::{get_content, main_menu};
use starter::{continue_project, manage_project, new_project};
use std::env;
use tracing::{debug, error};
use tracing_subscriber::{Layer, fmt, layer::SubscriberExt, reload, util::SubscriberInitExt};

fn main() -> Result<(), color_eyre::eyre::Report> {
    let rrogram_home = get_home::get_home() + "/.rrogram";
    color_eyre::install()?;

    // config for the output file of tracing
    let file = std::fs::File::create(rrogram_home.clone() + "/game.log")?;
    let file_layer = fmt::layer().with_writer(file).with_ansi(false);

    // config for the output to stdout of tracing
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    let (filter_layer, filter_layer_reload_handle) = reload::Layer::new(filter);
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(filter_layer);

    // init tracing
    tracing_subscriber::registry()
        .with(file_layer)
        .with(stdout_layer)
        .init();

    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let args = Args::parse();

    if let Some(this) = args.this {
        debug!("Running args command ...");
        match this {
            This::UninstallSelf => {
                if let Err(_) = std::fs::remove_dir_all(rrogram_home.as_str()) {
                    error!("No directory");
                    return Err(eyre!("No directory"));
                }
            }
        }
        debug!("Running args command ... ok");
        return Ok(());
    }

    println!("Rrogram v{}", VERSION);
    main_menu::show();
    loop {
        let user_input: String = get_content::get(None).expect("?");
        let user_input = user_input.as_str();

        let result = match user_input {
            "0" => continue_project::continue_project(
                rrogram_home.clone(),
                &filter_layer_reload_handle,
            ),
            "1" => new_project::new_project(rrogram_home.clone()),
            "2" => manage_project::manage_project(rrogram_home.clone()),
            "3" => break,
            _ => {
                error!("No such command");
                Ok(())
            }
        };
        if let Err(e) = result {
            eprintln!("{:?}", e);
        }
    }
    Ok(())
}
