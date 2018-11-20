use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread -> {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("main thread -> {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // 等待子线程执行完毕 -> 这里相当于将子线程附加到主线程给主线程续命. 附加时机也会导致线程执行顺序的不一致
}
