use std::error::Error;

pub trait ValueObject<T> {
  fn new(value: T) -> Result<T, Box<dyn Error>>;
}
