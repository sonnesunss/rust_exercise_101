use log::{self, info, warn};

fn main() {
    // 通过env_logger::init注册了一个logger后端，这个后端负责制真正的如何处理日志事件，例如
    // 打印到控制台，写入文件等
    env_logger::init();
    // info!, warn!只是macro， 它会被展开为调用log::log!(Level::Info, "log msg")的语句
    // 而log::log!(...)会调用全局设置的logger，logger通过set_logger()设置，例子通过env_logger注册
    info!("this is an info log");
    warn!("this is a warning");
}
