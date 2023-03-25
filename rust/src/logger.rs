use log::LevelFilter;
use log4rs::filter::Filter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};

#[derive(Debug)]
struct ExcludeMnemonicLogger;

impl Filter for ExcludeMnemonicLogger {
    fn filter(&self, record: &log::Record) -> log4rs::filter::Response {
        if record.target() == "mnemonic_logger" {
            log4rs::filter::Response::Reject
        } else {
            log4rs::filter::Response::Neutral
        }
    }
}

pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    // Set up terminal appender
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build();

    // Set up log file appender
    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("vanitygen.log")
        .unwrap();

    // Set up a second log file appender
    let mnemonic_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build("mnemonics.log")?;

    // Create logging configuration
    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ExcludeMnemonicLogger))
                .build("stdout", Box::new(stdout)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ExcludeMnemonicLogger))
                .build("log_file", Box::new(log_file)),
        )
        .appender(Appender::builder().build("mnemonic_log", Box::new(mnemonic_log)))
        .logger(
            Logger::builder()
                .appender("mnemonic_log")
                .build("mnemonic_logger", LevelFilter::Info),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("log_file")
                .build(LevelFilter::Info),
        )?;

    // Initialize the logger
    log4rs::init_config(config)?;

    Ok(())
}

#[macro_export]
macro_rules! mnemonic_log {
    ($($arg:tt)+) => {
        log::logger().log(
            &log::Record::builder()
                .level(log::Level::Info)
                .target("mnemonic_logger")
                .module_path(Some(module_path!()))
                .file(Some(file!()))
                .line(Some(line!()))
                .args(format_args!($($arg)+))
                .build()
        );
    };
}
