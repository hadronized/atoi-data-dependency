//! An interesting study of data dependencies (in a parsing loop).
//!
//! Here, atoi1 has a data dependency on output (it requires the previous output to be computed in
//! order to iterate through), while atoi2’s loop iterations are independent from each other. In
//! theory, the compiler should prefer atoi2 as it has more opportunity to parallelize the loop.
//! Looks like (at least in Rust via LLVM) that the first function benefits from the same kind
//! of optimization.
//!
//! Bench with:
//!   cargo bench
//!
//! Results (on my machine):
//!
//!   test tests::bench_atoi1 ... bench:          38 ns/iter (+/- 3)
//!   test tests::bench_atoi2 ... bench:          39 ns/iter (+/- 4)

#![feature(test)]

extern crate test;

pub fn atoi1(input: &str) -> Option<i64> {
  let mut bytes = input.as_bytes();
  let neg;

  if bytes[0] == b'-' {
    bytes = &bytes[1..];
    neg = true;
  } else {
    neg = false;
  }

  let mut output = 0;

  for c in bytes.iter() {
    let c = *c;

    if c < b'0' || c > b'9' {
      return None;
    }

    output = output * 10 + (c as i64 - '0' as i64);
  }

  let output = if neg { -output } else { output };
  Some(output)
}

pub fn atoi2(input: &str) -> Option<i64> {
  let mut bytes = input.as_bytes();
  let neg;

  if bytes[0] == b'-' {
    bytes = &bytes[1..];
    neg = true;
  } else {
    neg = false;
  }

  let mut output = 0;

  static POW10: [i64; 19] = [
    1000000000000000000,
    100000000000000000,
    10000000000000000,
    1000000000000000,
    100000000000000,
    10000000000000,
    1000000000000,
    100000000000,
    10000000000,
    1000000000,
    100000000,
    10000000,
    1000000,
    100000,
    10000,
    1000,
    100,
    10,
    1,
  ];

  for (i, c) in bytes.iter().rev().enumerate() {
    let c = *c;

    if c < b'0' || c > b'9' {
      return None;
    }

    output += (c as i64 - '0' as i64) * POW10[i];
  }

  let output = if neg { -output } else { output };
  Some(output)
}

#[cfg(test)]
mod tests {
  use test::Bencher;

  use super::*;

  #[bench]
  fn bench_atoi1(b: &mut Bencher) {
    b.iter(|| {
      let n = test::black_box(1000);

      (0..n).fold(false, |_, _| {
        atoi1("-1234567899876543210") == Some(-1234567899876543210)
      })
    })
  }

  #[bench]
  fn bench_atoi2(b: &mut Bencher) {
    b.iter(|| {
      let n = test::black_box(1000);

      (0..n).fold(false, |_, _| {
        atoi2("-1234567899876543210") == Some(-1234567899876543210)
      })
    })
  }
}
