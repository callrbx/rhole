use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::{
    fs::File,
    io::{self, copy},
    path::Path,
    time::Duration,
};

use crate::{common::Error, config::BlocklistConfig, entry::Entry};
use log::info;
use parse_duration;

pub struct Blocklist {
    pub name: String,
    pub url: String,
    pub update: Duration,
}

impl Blocklist {
    pub fn from_config(config: BlocklistConfig) -> Result<Self, Error> {
        let new_bl = Self {
            name: config.name,
            url: config.url,
            update: match parse_duration::parse(&config.update) {
                Ok(d) => d,
                Err(err) => {
                    return Err(Error::FailedToParseDur {
                        err: err.to_string(),
                    })
                }
            },
        };

        return Ok(new_bl);
    }

    fn download_list(self, filename: &str) -> Result<(), Error> {
        let target = self.url;
        let response = match reqwest::blocking::get(target) {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::FailedToParseDur {
                    err: err.to_string(),
                })
            }
        };

        let mut dest = File::create(filename).unwrap();

        let content = response.text().unwrap();
        match copy(&mut content.as_bytes(), &mut dest) {
            Ok(_) => {}
            Err(err) => {
                return Err(Error::FailedToDownload {
                    err: err.to_string(),
                })
            }
        }

        info!("Finished downloading");

        return Ok(());
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn normalize_line(entry: String) -> String {
        // strip comments and whitespace
        let host_entry = if entry.contains("#") {
            entry.split_once("#").unwrap().0.trim()
        } else {
            entry.trim()
        };

        return host_entry.to_string();
    }

    pub fn get_entries(self) -> Result<HashMap<String, Entry>, Error> {
        let mut entry_map: HashMap<String, Entry> = HashMap::new();
        let filename = "/tmp/tmp-bl.txt";
        let source = self.name.clone();

        self.download_list(&filename)?;

        let added = std::time::SystemTime::now();

        if let Ok(lines) = Self::read_lines(filename) {
            for line in lines {
                if let Ok(host_entry) = line {
                    if let Ok((host, entry)) =
                        Entry::from_line(&source, Self::normalize_line(host_entry), added, None)
                    {
                        entry_map.insert(host, entry);
                    }
                }
            }
        }

        match fs::remove_file(filename) {
            Ok(_) => {}
            Err(_) => eprintln!("Failed to remove tmp download"),
        }

        println!("Processed {} entries from {}", entry_map.len(), source);

        return Ok(entry_map);
    }
}
