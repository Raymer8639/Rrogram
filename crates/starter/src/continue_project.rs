use crate::project::Project;
use color_eyre::{
    self, Result,
    eyre::{Report, eyre},
};
use std::{fs::File, io::BufReader, path::Path};
use tracing::{debug, error, info};
use tracing_subscriber::{EnvFilter, Registry, fmt::format::DefaultFields, layer::Layered, reload};

pub fn continue_project(
    rrogram_home: String,
    filter_reload_handle: &reload::Handle<
        EnvFilter,
        Layered<
            tracing_subscriber::fmt::Layer<
                Registry,
                DefaultFields,
                tracing_subscriber::fmt::format::Format,
                File,
            >,
            Registry,
        >,
    >,
) -> Result<(), Report> {
    let project_name: String = menu::get_content::get(Some("name")).unwrap();
    let project_path = rrogram_home.clone() + "/saves/" + project_name.as_str();

    debug!("Finding the save: {}", project_path.clone());
    if !Path::new(&project_path).exists() {
        error!("Cannot find the save of the name is `{project_name}`");
        return Err(eyre!(
            "Cannot find the save of the name is `{project_name}`"
        ));
    }
    debug!("Finding the save ... ok");
    let project = match std::fs::File::open(project_path.clone() + "/main.json") {
        Ok(project) => project,
        Err(_) => {
            error!("File is incomplete: Cannot find main.json");
            return Err(eyre!("File is incomplete: Cannot find main.json"));
        }
    };
    info!("Reading save ...");
    let project = BufReader::new(project);
    let _project: Project = match serde_json::from_reader(project) {
        Ok(project) => project,
        Err(err) => {
            return Err(eyre!("{err}"));
        }
    };
    info!("Reading save ... ok");
    filter_reload_handle.modify(|f| *f = EnvFilter::new("off"))?;
    Ok(())
}
