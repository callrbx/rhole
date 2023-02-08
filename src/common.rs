use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    // Config Related Errors
    #[snafu(display("Unable to read config: {err}"))]
    FailedToReadConfig { err: String },
    #[snafu(display("Unable to parse duration: {err}"))]
    FailedToParseDur { err: String },

    // Hosts File Related Errors
    #[snafu(display("Unable to read hosts: {err}"))]
    FailedToReadHosts { err: String },
    #[snafu(display("Unable to write hosts: {err}"))]
    FailedToWriteHosts { err: String },

    // Download Related Errors
    #[snafu(display("Unable to download: {err}"))]
    FailedToDownload { err: String },

    // Entry Related Errors
    #[snafu(display("Unable to parse entry: {err}"))]
    EntryFormatError { err: String },
}
