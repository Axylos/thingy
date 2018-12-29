use log;
use syslog::{Facility, Formatter3164};

pub fn init() {
    syslog::init(syslog::Facility::LOG_USER,
                 log::LevelFilter::Debug,
                 Some("file system"));

    log::trace!("hey there!");
}
