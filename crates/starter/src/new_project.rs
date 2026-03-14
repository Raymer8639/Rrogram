use crate::project::Project;
use color_eyre::{
    Result,
    eyre::{Report, eyre},
};
use menu::get_content;
use std::{fs, path::Path};
use tracing::{debug, error};

pub fn new_project(rrogram_home: String) -> Result<(), Report> {
    let mut name: String;
    let mut save_path: String;
    loop {
        name = get_content::get(Some("name")).unwrap();
        if name.is_empty() {
            continue;
        }
        save_path = rrogram_home.clone() + "/saves/" + name.clone().as_str();
        if Path::new(&save_path.clone()).exists() {
            eprintln!("The name is the same as the name of other save files");
            continue;
        }
        break;
    }
    debug!("Creating the save ...");
    fs::create_dir_all(save_path.clone())?;
    debug!("Creating the save ... ok");
    debug!("Writing the json file ...");
    let main_struct = Project { wiget: vec![] };
    let main_json = match serde_json::to_string_pretty(&main_struct) {
        Ok(json) => json,
        Err(_) => {
            error!("Cannot init the json file");
            return Err(eyre!("Cannot init the json file"));
        }
    };
    match fs::write(save_path.clone() + "/main.json", main_json) {
        Ok(_) => (),
        Err(_) => {
            error!("Cannot create file");
            return Err(eyre!("Cannot create file"));
        }
    }
    debug!("Writing the json file ... Sucess");
    Ok(())
}
