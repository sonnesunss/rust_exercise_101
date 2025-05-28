/*
    # crossbeam_channel -- mpmc channel

    图示:

   +----------+  tx.send                                              rx.recv    +------------+
   |Producer A| -->--+                                              +--------->  | Consumer X |
   +----------+      |                                              |            +------------+
                     V            +-------------------+             ^
                     +-->--->---> | crossbeam channel | --->---->-->+
                     ^            +-------------------+             v
   +----------+      |                                              |           +------------+
   |Producer B| -->--+                                              +-------->  | Consumer Y |
   +----------+  tx.send                                             rx.recv    +------------+

*/

use crossbeam::channel::{bounded, select};
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个有界通道，容量为5
    let (s, r) = bounded(5);

    let num_producers = 3;
    let num_consumers = 2;
    let messages_per_producer = 10;

    // 生产者
    let mut producer_handles = Vec::new();
    for i in 0..num_producers {
        let sender = s.clone(); // 克隆发送端,允许多个生产者
        producer_handles.push(thread::spawn(move || {
            for j in 0..messages_per_producer {
                let msg = format!("Producer {} sending message {}", i, j);
                println!(
                    "{} -> sending: {}",
                    thread::current().name().unwrap_or("Unamed Producer"),
                    msg
                );
                sender.send(msg).unwrap(); // 发送消息，如果通道已满则阻塞
                thread::sleep(Duration::from_millis(50)); // 模拟生产时间
            }
            // 当生产者完成发送后，sender会自动drop
            // 当所有sender都被drop时，接收者将会收到Disconnection错误
        }));
    }

    // 消费者
    let mut consumer_handles = Vec::new();
    for _i in 0..num_consumers {
        let receiver = r.clone(); // 克隆多个接收端,允许多个消费者
        consumer_handles.push(thread::spawn(move || {
            loop {
                // 使用select！ macro 等待多个事件中的一个完成，其他的将被抛弃
                select! {
                    recv(receiver) -> msg => {
                        match msg {
                            Ok(m) => {
                                println!("{} <- Received: {}",
                                    thread::current().name().unwrap_or("Unnamed Consumer"), m);
                                thread::sleep(Duration::from_millis(100));
                            },
                            Err(e) => {
                                println!("{} <- channel Disconnected: {:?}",
                                    thread::current().name().unwrap_or("Unnamed Consumer"), e);
                                break;
                            }
                        }
                    },
                    // 哈可以添加其他分支，例如超时或来自其他通道的消息

                    // 如果在1秒内没有收到消息，则执行此分支
                    default(Duration::from_secs(1)) => {
                        println!("{} <- Mo message received for i second. Waiting...",
                            thread::current().name().unwrap_or("Unamed consumer"));
                    }
                }
            }
        }));
    }

    for handle in producer_handles {
        handle.join().unwrap();
    }

    // 手动关闭原始发送端，这样所有接收者才能最终收到Disconnected错误并退出循环
    // 如果不关闭，即使所有生产者线程都结束了，s仍然存在
    // 消费者会一直等待新消息而不会断开
    drop(s);

    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("\n\nAll producers and consumers have finished.");
}
