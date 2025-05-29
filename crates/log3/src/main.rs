use chrono::Local;
use fern::Dispatch;
use log::{debug, error, info, warn};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Dispatch::new()
        .level(log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{time}][{level}][{target}][{file}]:[{line}][{threadn}] {msg}",
                time = Local::now().format("%Y-%m-%d %H:%M:%S.3f"),
                level = record.level(),
                target = record.target(),
                file = record.file().unwrap_or("unknown"),
                line = record.line().unwrap_or(0),
                threadn = thread::current().name().unwrap_or("Unnamed"),
                msg = message,
            ))
        })
        .chain(std::io::stdout())
        .apply()?;

    info!("服务器启动完成");
    warn!("警告： 配置文件未找到");
    debug!("调试信息: 变量 x = {}", 42);
    error!("错误: 数据库连接失败");
    Ok(())
}
