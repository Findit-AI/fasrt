/// Returns the number of digits in the decimal representation of `n`.
#[cfg_attr(not(tarpaulin), inline(always))]
pub const fn u64_digits(n: u64) -> usize {
  if n == 0 { 1 } else { (n.ilog10() + 1) as usize }
}
