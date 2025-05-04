use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

// 对QOTD协议的简单实现，基于tcp
use rand;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000);
    let server = TcpListener::bind(socket_addr)?;

    loop {
        match server.accept() {
            Ok((mut tcp_stream, addr)) => {
                println!("New connection is coming {addr}");

                let buf = retrie_msg().as_bytes();

                if let Err(e) = tcp_stream.write_all(buf) {
                    eprintln!("write failed with error ->{e}");
                }
                tcp_stream.flush()?;
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
}

#[allow(dead_code)]
fn retrie_msg() -> &'static str {
    let quotes = [
        "生活是一场冒险，勇敢迈出第一步。",
        "智慧源于倾听，力量来自行动。",
        "每一天都是新的开始，抓住它。",
        "梦想是风，行动是帆。",
        "简单的心，复杂的世界。",
        "坚持不懈，直到星辰坠落。",
        "时间是最好的老师，但从不留情。",
        "勇敢面对未知，奇迹自会显现。",
        "心有多大，世界就有多宽。",
        "失败是成功的垫脚石。",
        "用微笑迎接每一个黎明。",
        "信念如山，移山填海。",
        "人生如戏，全凭演技。",
        "沉默是金，行动是钻石。",
        "热爱生活，生活也会爱你。",
        "每一次跌倒，都是为了更高地飞翔。",
        "选择比努力更重要。",
        "心若向阳，无畏风霜。",
        "世界很大，去看看吧。",
        "做自己的光，不必等风来。",
        "时间会证明一切，耐心等待。",
        "平凡中孕育伟大。",
        "脚步不停，梦想不灭。",
        "用汗水浇灌希望的种子。",
        "生活是画布，你是画家。",
        "勇敢做梦，疯狂追梦。",
        "没有伞的孩子，跑得更快。",
        "眼中有星辰，心中有大海。",
        "成功是无数个小努力的积累。",
        "活在当下，爱在心间。",
    ];

    let rand = rand::random_range(0..quotes.len());

    quotes[rand]
}
