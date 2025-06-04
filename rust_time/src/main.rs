// 表示一个时间距离， 注意是关于时间的距离，而不是具体的时间点
use std::time::Duration;
// 表示一个单调非递减时钟的测量点，这个时间点可以是具体的过去的、未来的时间点， 只可以增加，不受系统时间调整的影响
use std::time::Instant;

fn main() {
    let duration1 = Duration::new(10, 1);
    let duration2 = Duration::from_secs(20);
    let duration3 = Duration::from_millis(30);

    let duration4 = duration1.checked_add(duration2);

    let _duration5 = duration3.checked_sub(duration3);

    let instant1 = Instant::now();
    let duration6 = instant1.elapsed();
    let instant3 = instant1.duration_since(Instant::now());
}
