use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;

pub fn logger() -> Result<(), Box<dyn Error>> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%d.%m.%Y %H:%M:%S)} | {({l}):5.5} | {m}{n}\n")))
        .build(format!("log/output-{}.log", SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()))?;

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%d.%m.%Y %H:%M:%S)} | {({l}):5.5} | {m}{n}\n")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder()
            .appender("logfile")
            .appender("stdout")
            .build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    Ok(())
}