use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

#[napi]
pub fn bin_u64(bin: Buffer) -> i64 {
  u64::from_le_bytes(bin[..].try_into().unwrap()) as _
}

#[napi]
pub fn u64_bin(i: i64) -> Buffer {
  (i as u64).to_le_bytes()[..].into()
}
