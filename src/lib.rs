use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;
use ordered_varint::Variable;

#[napi]
pub fn bin_u64(bin: Uint8Array) -> i64 {
  u64::decode_variable(&bin[..]).unwrap() as _
}

#[napi]
pub fn u64_bin(i: i64) -> Uint8Array {
  (i as u64).to_variable_vec().unwrap().into()
}
