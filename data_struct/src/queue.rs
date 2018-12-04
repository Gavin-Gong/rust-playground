#[derive(Debug)]
struct Queue<T> {
  data: Vec<T>,
}

impl<T> Queue<T> {
  fn new() -> Self {
    Queue { data: Vec::new() }
  }

  fn push(&mut self, item: T) {
    self.data.push(item);
  }

  fn pop(&mut self) -> Option<T> {
    if self.data.len() > 0 {
      Some(self.data.remove(0))
    } else {
      None
    }
  }
}
#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_Queue() {
    let mut q = Queue::new();
    q.push(1);
    q.push(2);
    assert_eq!(q.pop(), Some(1));
    assert_eq!(q.pop(), Some(2));
    assert_eq!(q.pop(), None);
  }
}
