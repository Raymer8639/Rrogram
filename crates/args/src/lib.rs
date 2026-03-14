use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub this: Option<This>,
}

#[derive(Subcommand)]
pub enum This {
    UninstallSelf,
}
