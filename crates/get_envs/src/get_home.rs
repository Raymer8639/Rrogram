use std::process;

pub fn get_home() -> String {
    let result = home::home_dir()
        .unwrap_or_else(|| {
            println!("Error: Cannot get the `$HOME` directory");
            process::exit(1);
        })
        .display()
        .to_string();
    println!("Home: {}", result.clone());
    result
}
