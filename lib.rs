#[crate_id="rust-mysql-simple#0.9.0.0"];
#[comment="Mysql client library writen in rust"];
#[license="MIT"];
#[crate_type="rlib"];
#[crate_type="dylib"];

#[allow(dead_code)];

extern mod extra;

pub mod consts;
pub mod sha1;
pub mod scramble;
pub mod conn;