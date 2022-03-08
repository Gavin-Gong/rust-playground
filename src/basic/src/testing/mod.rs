
pub fn t() -> usize {
  return 2;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(t(), 2);
    }
}
