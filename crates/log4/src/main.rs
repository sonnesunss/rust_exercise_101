use chrono::Local;
use fern::Dispatch;
use log::{self, debug, error, info, warn};
use std::fs;
use std::io;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取当前日期生成文件名，例如: logs/2025-05-29.log
    let date = Local::now().format("%Y-%md-%d").to_string();
    // 如果没有logs目录，则创建之
    fs::create_dir("logs")?;
    let log_file_path = format!("logs/{}..log", date);
    let log_file = fern::log_file(log_file_path)?;

    // 设置日志格式和输出
    Dispatch::new()
        .level(log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{time}][{level}][{target}][{file}]:[{line}][{thread_name}] {msg}",
                time = Local::now().format("%Y-%m-%d %H-%M-%S%.3f"),
                level = record.level(),
                target = record.target(),
                file = record.file().unwrap_or("Unknown"),
                line = record.line().unwrap_or(0),
                thread_name = thread::current().name().unwrap_or("Unnamed"),
                msg = message,
            ))
        })
        .chain(io::stdout())
        .chain(log_file)
        .apply()?;

    info!("服务启动完成");
    warn!("配置文件未找到");
    debug!("连接池启动成功");
    error!("数据库连接失败");

    Ok(())
}
