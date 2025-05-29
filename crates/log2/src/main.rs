use chrono::Local;
use log::{self, error, info, warn};

fn main() {
    set_logger().expect("Failed to initialize logger");

    info!("Application started");
    warn!("This is a warning");
    error!("Something went wrong!");
}

fn set_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .level(log::LevelFilter::Info) // 过滤至少Info等级的日志
        .chain(
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}][{}] {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(std::io::stdout()),
        )
        .chain(
            fern::Dispatch::new()
                .filter(|metadata| metadata.level() == log::Level::Error)
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}][{}] {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(fern::log_file("error.log")?),
        )
        .apply()?;

    Ok(())
}
