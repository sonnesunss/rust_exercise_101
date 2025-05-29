// 支持每日自动轮转，自动切换到当日日期的文件日志中
// 支持结构化字段
use std::time::Duration;
use tracing::{debug, error, info, warn};
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn main() {
    // 1. 创建按时轮换的文件appender， 路径为 logs/daily_date.log
    let file_appender: RollingFileAppender = tracing_appender::rolling::daily("logs", "server.log");

    // 2. 构建一个非阻塞、线程安全的writer
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);

    // 3. 设置日志过滤级别， 支持环境变量动态设置
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    // 4. 初始化tracing subscriber，输出到stdout + file
    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::Layer::new()
                .with_writer(std::io::stdout)
                .with_ansi(true)
                .with_timer(fmt::time::ChronoLocal::rfc_3339()),
        )
        .with(
            fmt::Layer::new()
                .with_writer(non_blocking_file)
                .with_ansi(true)
                .with_timer(fmt::time::ChronoLocal::rfc_3339()),
        )
        .init();

    info!("服务启动完成");
    warn!(code = 1001, "配置文件未找到");
    debug!(host = "localhost", port = 8080, "服务监听中");
    error!("数据库连接失败");

    std::thread::sleep(Duration::from_secs(10));
}
