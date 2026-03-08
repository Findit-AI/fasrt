use std::{
  env::{self, var},
  fs::OpenOptions,
  io::Write,
  path::PathBuf,
};

fn main() {
  // Don't rerun this on changes other than build.rs, as we only depend on
  // the rustc version.
  println!("cargo:rerun-if-changed=build.rs");

  // Check for `--features=tarpaulin`.
  let tarpaulin = var("CARGO_FEATURE_TARPAULIN").is_ok();

  if tarpaulin {
    use_feature("tarpaulin");
  } else {
    // Always rerun if these env vars change.
    println!("cargo:rerun-if-env-changed=CARGO_TARPAULIN");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARPAULIN");

    // Detect tarpaulin by environment variable
    if env::var("CARGO_TARPAULIN").is_ok() || env::var("CARGO_CFG_TARPAULIN").is_ok() {
      use_feature("tarpaulin");
    }
  }

  generate_from_str_60("minute");
  generate_to_str_60("minute");

  generate_from_str_60("second");
  generate_to_str_60("second");

  generate_hour_from_str();
  generate_hour_to_str();

  generate_millis_from_str();
  generate_millis_to_str();

  // Rerun this script if any of our features or configuration flags change,
  // or if the toolchain we used for feature detection changes.
  println!("cargo:rerun-if-env-changed=CARGO_FEATURE_TARPAULIN");
}

fn use_feature(feature: &str) {
  println!("cargo:rustc-cfg={}", feature);
}

fn generate_from_str_60(name: &str) {
  let mut out = String::new();

  for i in 0..10 {
    out.push_str(&format!(
      "      \"{}\" => return ::core::result::Result::Err(Self::Err::NotPadded),\n",
      i
    ));
  }

  for i in 0..60 {
    out.push_str(&format!("      \"{:02}\" => {},\n", i, i));
  }

  let output = format!(
    r###"
macro_rules! {name}_from_str {{
  ($value:expr) => {{{{
    ::core::result::Result::Ok(Self(match $value {{
{}     _ => {{
        let val = <::core::primitive::u8 as ::core::str::FromStr>::from_str($value)?;
        return ::core::result::Result::Err(Self::Err::Overflow(val));
      }},
    }}))
  }}}}
}}

pub(super) use {name}_from_str;
"###,
    out
  );

  let path = PathBuf::from("generated").join(format!("{}_from_str.rs", name));
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .expect("failed to open generated file");
  file
    .write_all(output.as_bytes())
    .expect("failed to write generated file");
}

fn generate_to_str_60(name: &str) {
  let mut out = String::new();

  for i in 0..60 {
    out.push_str(&format!("      {} => \"{:02}\",\n", i, i));
  }

  let output = format!(
    r###"
macro_rules! {name}_to_str {{
  ($value:expr) => {{{{
    match $value {{
{}     _ => ::core::panic!("{} value must be between 00-59"),
    }}
  }}}}
}}

pub(super) use {name}_to_str;
"###,
    out, name
  );

  let path = PathBuf::from("generated").join(format!("{}_to_str.rs", name));
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .expect("failed to open generated file");
  file
    .write_all(output.as_bytes())
    .expect("failed to write generated file");
}

fn generate_millis_from_str() {
  let mut out = String::new();

  for i in 0..10 {
    out.push_str(&format!(
      "      \"{}\" => return ::core::result::Result::Err(Self::Err::NotPadded),\n",
      i
    ));
  }

  for i in 0..100 {
    out.push_str(&format!(
      "      \"{:02}\" => return ::core::result::Result::Err(Self::Err::NotPadded),\n",
      i
    ));
  }

  for i in 0..1000 {
    out.push_str(&format!("      \"{:03}\" => {},\n", i, i));
  }

  let output = format!(
    r###"
macro_rules! millisecond_from_str {{
  ($value:expr) => {{{{
    ::core::result::Result::Ok(Self(match $value {{
{}     _ => {{
        let val = <::core::primitive::u16 as ::core::str::FromStr>::from_str($value)?;
        return ::core::result::Result::Err(Self::Err::Overflow(val));
      }},
    }}))
  }}}}
}}

pub(super) use millisecond_from_str;
"###,
    out
  );

  let path = PathBuf::from("generated").join("millisecond_from_str.rs");
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .expect("failed to open generated file");
  file
    .write_all(output.as_bytes())
    .expect("failed to write generated file");
}

fn generate_millis_to_str() {
  let mut out = String::new();

  for i in 0..1000 {
    out.push_str(&format!("      {} => \"{:03}\",\n", i, i));
  }

  let output = format!(
    r###"
macro_rules! millisecond_to_str {{
  ($value:expr) => {{{{
    match $value {{
{}     _ => ::core::panic!("Millisecond value must be between 000-999"),
    }}
  }}}}
}}

pub(super) use millisecond_to_str;
"###,
    out,
  );

  let path = PathBuf::from("generated").join("millisecond_to_str.rs");
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .expect("failed to open generated file");
  file
    .write_all(output.as_bytes())
    .expect("failed to write generated file");
}

fn generate_hour_from_str() {
  let mut out = String::new();

  for i in 0..10 {
    out.push_str(&format!(
      "      \"{}\" => return ::core::result::Result::Err(Self::Err::NotPadded),\n",
      i
    ));
  }

  for i in 0..100 {
    out.push_str(&format!("      \"{i:02}\" => {i},\n"));
  }

  for i in 100..1000 {
    out.push_str(&format!("      \"{i}\" => {i},\n"));
  }

  let output = format!(
    r###"
macro_rules! hour_from_str {{
  ($value:expr) => {{{{
    ::core::result::Result::Ok(Self(match $value {{
{}     _ => {{
        let val = <::core::primitive::u16 as ::core::str::FromStr>::from_str($value)?;
        return ::core::result::Result::Err(Self::Err::Overflow(val));
      }},
    }}))
  }}}}
}}

pub(super) use hour_from_str;
"###,
    out
  );

  let path = PathBuf::from("generated").join("hour_from_str.rs");
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .expect("failed to open generated file");
  file
    .write_all(output.as_bytes())
    .expect("failed to write generated file");
}

fn generate_hour_to_str() {
  let mut out = String::new();

  for i in 0..1000 {
    if i == 0 {
      out.push_str(&format!("{} => \"{:02}\",\n", i, i));
      continue;
    }

    out.push_str(&format!("      {} => \"{:02}\",\n", i, i));
  }

  let output = format!(
    r###"
macro_rules! hour_to_str {{
  ($value:expr) => {{{{
    match $value {{
{}     _ => ::core::panic!("Hour value must be between 00-999"),
    }}
  }}}}
}}

pub(super) use hour_to_str;
"###,
    out,
  );

  let path = PathBuf::from("generated").join("hour_to_str.rs");
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .expect("failed to open generated file");
  file
    .write_all(output.as_bytes())
    .expect("failed to write generated file");
}
