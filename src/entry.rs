use std::time::SystemTime;

use log::info;

use crate::common::Error;

pub struct Entry {
    pub ip: String,
    pub host: String,
    pub expiry: SystemTime,
    pub added: SystemTime,
    pub source: String,
}

impl Entry {
    pub fn new(
        ip: String,
        host: String,
        expiry: Option<SystemTime>,
        added: SystemTime,
        source: String,
    ) -> Self {
        return Self {
            ip: ip,
            host: host,
            expiry: match expiry {
                Some(e) => e,
                None => SystemTime::UNIX_EPOCH,
            },
            added: added,
            source: source,
        };
    }

    pub fn from_line(
        source: &str,
        host_entry: String,
        added: SystemTime,
        expiry: Option<SystemTime>,
    ) -> Result<(String, Self), Error> {
        let line_splits = host_entry.split_ascii_whitespace().collect::<Vec<&str>>();

        let (ip, host) = match line_splits.len() {
            1 => ("0.0.0.0".to_string(), line_splits[0].to_string()),
            2 => (line_splits[0].to_string(), line_splits[1].to_string()),
            _ => {
                return Err(Error::EntryFormatError {
                    err: host_entry.to_string(),
                })
            }
        };

        info!("{}: {}", ip, host);

        return Ok((
            host.clone(),
            Self::new(ip, host, expiry, added, source.to_string()),
        ));
    }
}
