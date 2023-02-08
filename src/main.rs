use std::{
    collections::HashMap,
    fs::{self},
};

use common::Error;
use dnsdb::DBArgs;
use entry::Entry;
use structopt::StructOpt;

use crate::blocklist::Blocklist;

mod blocklist;
mod common;
mod config;
mod dnsdb;
mod entry;

const DELIM_PATTERN: &str = "# ___rhole___\n";

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    about = "simple download and rewrite of hosts",
    author = "drew <drew@parker.systems>"
)]
pub struct BasicArgs {
    #[structopt(short = "c", long = "config", help = "config file to parse")]
    pub config: String,
    #[structopt(
        short = "i",
        long = "input",
        help = "input file to read",
        default_value = "/etc/hosts"
    )]
    pub input: String,
    #[structopt(
        short = "o",
        long = "output",
        help = "output file to write",
        default_value = "/etc/hosts"
    )]
    pub output: String,
    #[structopt(short = "f", long = "force", help = "overwrite existing /etc/hosts")]
    pub overwrite: bool,
}

/// Declare submodule argument types for matching
#[derive(Debug, StructOpt, Clone)]
pub enum Command {
    Basic(BasicArgs),
    DBMgmt(DBArgs),
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "rhole",
    about = "hosts based DNS blocking manager",
    author = "drew <drew@parker.systems>"
)]
pub struct Args {
    // Sub Command
    #[structopt(help = "rhole sub-command", subcommand)]
    pub command: Command,
}

fn basic_handler(args: BasicArgs) -> Result<(), common::Error> {
    let conf = config::parse_config_file(args.config)?;
    let mut contents = String::new();

    if !args.overwrite {
        match fs::read_to_string(args.input) {
            Ok(c) => contents.push_str(&c),
            Err(err) => {
                return Err(Error::FailedToReadHosts {
                    err: err.to_string(),
                })
            }
        }
        contents = match contents.split_once(DELIM_PATTERN) {
            Some((saved, _)) => saved.to_string(),
            None => contents,
        }
    }
    contents.push_str(DELIM_PATTERN);

    let mut entries: HashMap<String, Entry> = HashMap::new();

    for blconf in conf.blocklist {
        let bl = Blocklist::from_config(blconf)?;

        println!("Processing {}", bl.name);
        entries.extend(bl.get_entries()?);
    }

    for (host, entry) in entries.iter() {
        contents.push_str(&format!("{} {}\n", entry.ip, host));
    }

    return match fs::write(args.output, contents) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::FailedToWriteHosts {
            err: err.to_string(),
        }),
    };
}

fn main() -> Result<(), common::Error> {
    env_logger::init();

    let args = Args::from_args();

    let res = match args.command {
        Command::Basic(mod_args) => basic_handler(mod_args),
        Command::DBMgmt(mod_args) => dnsdb::handler(mod_args),
    };

    return res;
}
