use menu::{get_content, main_menu};
use tracing::debug;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    debug!("App initing");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("Rrogram v{}", VERSION);
    debug!("App started");
    main_menu::show();
    loop {
        let user_input: String = get_content::get(None).expect("?");
        let user_input = user_input.as_str();
        match user_input {
            "0" => {}
            "1" => {}
            "2" => {}
            "3" => {
                break;
            }
            _ => {
                println!("?");
            }
        }
    }
}
