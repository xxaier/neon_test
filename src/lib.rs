use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use ordered_varint::Variable;

#[napi]
pub fn bin_u64(bin: Buffer) -> i64 {
  u64::decode_variable(&bin[..]).unwrap() as _
}

#[napi]
pub fn u64_bin(i: i64) -> Buffer {
  Buffer::from((i as u64).to_variable_vec().unwrap())
}
