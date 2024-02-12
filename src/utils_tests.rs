#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use crate::utils::{convert_runtime, convert_to_gb};

  #[test]
  fn test_convert_to_gb() {
    assert_eq!(convert_to_gb(2147483648), 2f64);
    assert_eq!(convert_to_gb(2662879723), 2.4799999995157123);
  }

  #[test]
  fn test_convert_runtime() {
    let (hours, minutes) = convert_runtime(154);

    assert_eq!(hours, 2);
    assert_eq!(minutes, 34);
  }
}
