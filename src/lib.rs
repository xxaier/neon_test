use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

#[napi]
pub fn bin_i64(bin: Buffer) -> i64 {
  i64::from_le_bytes(bin[..].try_into().unwrap())
}

#[napi]
pub fn i64_bin(i: i64) -> Buffer {
  (i as i64).to_le_bytes()[..].into()
}
