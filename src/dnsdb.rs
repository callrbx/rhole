use log::info;
use structopt::StructOpt;

use crate::{blocklist::Blocklist, common, config};

// Init Subcommand Args
#[derive(Debug, StructOpt, Clone)]
pub struct InitArgs {
    #[structopt(short = "c", long = "config", help = "config file to parse")]
    pub config: String,
}

/// Declare submodule argument types for matching
#[derive(Debug, StructOpt, Clone)]
pub enum Command {
    Init(InitArgs),
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    about = "manage the underlying DB",
    author = "drew <drew@parker.systems>"
)]
pub struct DBArgs {
    // Sub Command
    #[structopt(help = "db sub-command", subcommand)]
    pub command: Command,
}

fn init_db(args: InitArgs) -> Result<(), common::Error> {
    info!("Creating DB from config: {}", args.config);

    let conf = config::parse_config_file(args.config)?;

    for blconf in conf.blocklist {
        let bl = Blocklist::from_config(blconf)?;

        println!("Processing {}", bl.name);
        bl.get_entries()?;
    }

    return Ok(());
}

// sub command handler for the db-mgmt interfaces
pub fn handler(args: DBArgs) -> Result<(), common::Error> {
    match args.command {
        Command::Init(args) => init_db(args)?,
    }

    return Ok(());
}
