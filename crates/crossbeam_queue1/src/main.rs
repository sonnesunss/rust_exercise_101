/*
    # crossbeam_queue crates introduce

    高性能的无锁MPMC queue

    + ArrayQueue<T> 是一个有界的MPMC队列，它在创建时分配一个固定容量的缓冲区，当队列满时，push操作会返回一个错误，而不是阻塞.

    + SeqQueue<T> 则是一个crossbeam_queue提供的无界MPMC队列，它在需要时动态分配小的段segments来存储元素，由于需要动态分配， 它通常比ArrayQueue略慢.

crossbeam_queue 提供的这些队列在需要高性能、无锁或非阻塞的并发队列时非常有用，它们在内部通过精巧的原子操作实现线程安全，避免了传统锁带来的性能瓶颈


    本例子的图示:

    +------------+
    | Producer A |  ->----------push------->--------+
    +------------+                                  |
                                                    |
    +------------+                                  |                                  +-------------+
    | Producer B |  ->----------push------->-----+  |          +------>----pop------>  | Consumer Z  |
    +------------+                               |  |          |                       +-------------+
                                                 v  v          ^
                                            +---------------------+                   +-------------+
                                            |  无锁队列ArrayQueue | -->-----pop-----> | Consumer X  |
                                            +---------------------+                   +-------------+
                                                ^   ^          v
                                                |   |          |
    +------------+                              |   |          |
    | Producer C |  ->---------push------->-----+   |          |                      +-------------+
    +------------+                                  |          +----->------pop-----> | Consumer Y  |
                                                    |                                 +-------------+
    +------------+                                  |
    | Producer N |  ->---------push------->---------+
    +------------+

    模拟一个生产者-消费者场景，其中多个生产者线程向一个有界队列中添加任务，而多个消费者线程从队列中获取并处理任务,
    这一切都是并发执行的

*/

use crossbeam_queue::ArrayQueue;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const QUEUE_CAPACITY: usize = 20;
const NUM_PRODUCERS: usize = 5;
const NUM_CONSUMERS: usize = 3;
const MSG_PER_PRODUCER: usize = 50;

fn main() {
    // 创建一个可以在多线程之间共享所有权的crossbeam mpmc queue
    let queue = Arc::new(ArrayQueue::new(QUEUE_CAPACITY));

    let mut producer_handles = Vec::new();
    for i in 0..NUM_PRODUCERS {
        let q = Arc::clone(&queue);

        producer_handles.push(thread::spawn(move || {
            for j in 0..MSG_PER_PRODUCER {
                let mut task = format!("Task from Producer {} - {}", i, j);
                loop {
                    match q.push(task.clone()) {
                        Ok(_) => {
                            println!("Producer {} pushed: {}", i, task);
                            break; // 成功推入则跳出循环
                        }
                        Err(t) => {
                            // 为什么需要外面的循环，因为推入失败会返回一个错误，loop是遇到错误重试
                            println!("Producer {} queue full, retrying for: {}", i, t);
                            thread::sleep(Duration::from_millis(10));
                            // 继续尝试推入相同的任务
                            task = t; // 将未推入的任务重新赋值，以便下次循环重试直到成功推入
                        }
                    }
                }
            }
            println!("Producer {} finished.", i);
        }));
    }

    let mut consumer_handles = Vec::new();
    for i in 0..NUM_CONSUMERS {
        let q = Arc::clone(&queue);

        consumer_handles.push(thread::spawn(move || {
            let mut received_count = 0;
            loop {
                match q.pop() {
                    Some(task) => {
                        println!("Consumer {} processed: {}", i, task);
                        received_count += 1;
                        thread::sleep(Duration::from_millis(20));
                    }
                    None => {
                        // 简单的检查队列为空时就认为所有生产者已经全部完成，很粗暴简单
                        // 实际健壮代码需要额外的处理，使其更可靠一点
                        // 例如使用额外的通道发出结束信号
                        // 使用一个原子计数器
                        // 使用一个单独的毒丸等
                        if received_count >= NUM_PRODUCERS * MSG_PER_PRODUCER / NUM_CONSUMERS {
                            println!("Consumer {} suspects no more tasks, exiting.", i);
                            break;
                        }
                        println!("Consumer {} queue empty, waiting...", i);
                        thread::sleep(Duration::from_millis(50)); // 等待一段时间后重试
                    }
                }
            }
        }));
    }

    for handle in producer_handles {
        handle.join().unwrap();
    }

    println!("\n\nAll producers finished. Consumers will continue to drain the queue");

    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("\nAll producers and consumers have finished");
    println!("Final queue size: {}", queue.len());
}
