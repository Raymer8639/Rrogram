use crate::project::Project;
use menu::get_content;
use std::{fs, path::Path};
use tracing::debug;

pub fn new_project(rrogram_home: String) {
    let mut name: String;
    let mut save_path: String;
    loop {
        name = get_content::get(Some("name")).unwrap();
        if name == "".to_string() {
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
    fs::create_dir_all(save_path.clone()).unwrap();
    debug!("Creating the save ... ok");
    debug!("Writing the json file ...");
    let main_struct = Project { wiget: vec![] };
    let main_json = serde_json::to_string_pretty(&main_struct).expect("Cannot init the json file");
    fs::write(save_path.clone() + "/main.json", main_json).expect("Cannot create file");
    debug!("Writing the json file ... Sucess");
}
