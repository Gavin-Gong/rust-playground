/// 互斥器的使用
///
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
  let x = Arc::new(Mutex::new(5));
  let mut handles = vec![];
  // 11 个线程
  for _ in 0..10 {
    let x = Arc::clone(&x);
    let handle = thread::spawn(move || {
      let mut num = x.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
  println!("{:?}", x);
}

// 1. Arc 多所有权 引用计数
// 2. 上锁需手动进行, 解锁由 Rust 自动完成
