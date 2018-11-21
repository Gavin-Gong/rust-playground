use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![String::from("Hello"), String::from("World")];
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_millis(100));
    }
  });

  // rx.recv().unwrap()
  // 获取传递的值
  for recved in rx {
    println!("Got {}", recved);
  }
}

// 上面例子还只是 单对单的发送消息, 如果想要多对一的话
// 需要对发送者进行 clone
// mpsc::Sender::clone(&tx);
