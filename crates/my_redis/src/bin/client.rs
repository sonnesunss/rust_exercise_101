use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[allow(dead_code)]
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: oneshot::Sender<mini_redis::Result<Option<Bytes>>>, // 顺带携带一个发送者进去
    },
    Set {
        key: String,
        val: Bytes,
        resp: oneshot::Sender<mini_redis::Result<()>>,
    },
}

#[tokio::main]
async fn main() {
    // 创建有界通道，多发送者互不干扰对方，这意味着多个任务可以同时发送消息
    let (tx, mut rx) = mpsc::channel::<Command>(32);
    let tx2 = tx.clone();

    // 多个任务发送命令互不干扰
    // set task
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx.send(Command::Set {
            key: "hello".to_string(),
            val: "world".into(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let res = resp_rx.await;

        println!("got = {:?}", res);
    });

    // get task
    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        tx2.send(Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let res = resp_rx.await;

        println!("got = {:?}", res);
    });

    // manager task
    // 需要一个总的任务管理任务，这些儿任务的作用是以redis客户端的身份连接到redis服务器，然后启动rx接收其他任务发送来的它想发送给服务器的命令消息，
    // 然后管理任务的任务会代替它们通过redis连接发送给redis服务器
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // 开始接收命令消息
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, resp } => {
                    let r = client.get(&key).await;
                    let _ = resp.send(r);
                    //println!("receved data from redis server is -> {:?}", r);
                    // get命令获得结果后，还需要传递给交给它的那个任务， 告诉它结果是什么
                    // 因为要一对一的通知，且只发送一次，oneshot channel是个不错的选择
                    // 对于get而言，发起get的任务需要知道获取键对应的值，
                    // set命令需要知道操作是否成功
                    // 为了能够从 消息管理任务接收到响应，oneshot通道会在命令发送前创建， 其发送者被包含在
                    // 命令中发送到 消息管理任务中. 为了实现这个，在前面Command定义里面case分别添加上一个表示发送者的字段就好
                }
                Set { key, val, resp } => {
                    let r = client.set(&key, val).await;
                    let _ = resp.send(r);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
