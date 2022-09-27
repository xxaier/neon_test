mod bin;
mod r#macro;
mod r#trait;

use std::{
  hash::{BuildHasher, Hasher},
  net::IpAddr,
};

use anyhow::Result;
use base64_simd::Base64;
use napi::{
  bindgen_prelude::{AsyncTask, Uint8Array},
  Env, Task,
};
use napi_derive::napi;
use ordered_varint::Variable;
use xxhash_rust::xxh3::Xxh3Builder;

use crate::bin::{Bin, Bins};

#[napi]
pub fn z85_dump(bin: Bin) -> String {
  z85::encode(bin)
}

#[napi]
pub fn z85_load(s: Bin) -> Result<Uint8Array> {
  Ok(z85::decode(s)?.into())
}

#[napi]
pub fn ip_bin(ip: String) -> Result<Uint8Array> {
  Ok(match ip.parse()? {
    IpAddr::V4(ip) => {
      let o = ip.octets();
      [o[0], o[1], o[2], o[3]].into()
    }
    IpAddr::V6(ip) => {
      let o = ip.octets();
      [
        o[0], o[1], o[2], o[3], o[4], o[5], o[6], o[7], o[8], o[9], o[10], o[11], o[12], o[13],
        o[14], o[15],
      ]
      .into()
    }
  })
}

#[napi]
pub fn bin_u64(bin: Uint8Array) -> i64 {
  u64::decode_variable(&bin[..]).unwrap() as _
}

#[napi]
pub fn u64_bin(i: i64) -> Uint8Array {
  (i as u64).to_variable_vec().unwrap().into()
}

#[napi]
pub fn zip_u64(li: Vec<i64>) -> Uint8Array {
  let li = li.into_iter().map(|i| i as u64).collect::<Vec<_>>();
  let len = li.len();
  let data_len = stream_vbyte64::max_compressed_len(len);
  let mut buf = vec![0; data_len];
  let len = stream_vbyte64::encode(&li, &mut buf);
  buf[..len].into()
}

#[napi]
pub fn unzip_u64(li: Uint8Array, len: i64) -> Vec<i64> {
  dbg!(&li[..], len);
  let mut decoded = vec![0; len as usize];
  stream_vbyte64::decode(&mut decoded, &li[..]);
  decoded.into_iter().map(|i| i as i64).collect()
}

const BASE64: Base64 = Base64::URL_SAFE_NO_PAD;

#[napi]
pub fn b64(input: Bins) -> String {
  let mut vec = vec![];
  for i in input {
    vec.extend_from_slice(i.as_ref())
  }
  BASE64.encode_to_boxed_str(&vec).into()
}

#[napi]
pub fn unb64(input: Bin) -> Result<Uint8Array> {
  Ok(BASE64.decode_to_boxed_bytes(input.as_ref())?.into())
}

struct HashRound {
  round: u32,
  data: Option<Bins>,
}

impl Task for HashRound {
  type Output = Vec<u8>;
  type JsValue = Uint8Array;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    let mut hasher = blake3::Hasher::new();
    {
      for i in self.data.take().unwrap() {
        hasher.update(i.as_ref());
      }
    }

    let mut output = [0; 2048];
    for _ in 1..self.round {
      hasher.finalize_xof().fill(&mut output);
      hasher.update(&output);
    }

    Ok((&hasher.finalize().as_bytes()[..]).into())
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi]
fn blake3_round(data: Bins, round: u32) -> AsyncTask<HashRound> {
  AsyncTask::new(HashRound {
    data: Some(data),
    round,
  })
}

#[napi]
pub fn blake3(input: Bins) -> Uint8Array {
  let mut hasher = blake3::Hasher::new();
  for i in input {
    hasher.update(i.as_ref());
  }
  (&hasher.finalize().as_bytes()[..]).into()
}

const XXHASHER: Xxh3Builder = Xxh3Builder::new();

#[napi]
pub fn xxh3(input: Bins) -> u64 {
  let mut h64 = XXHASHER.build_hasher();
  for i in input {
    h64.update(i.as_ref());
  }
  h64.finish()
}

#[napi]
pub fn encrypt(secret: Bin, iv: Bin, data: Bin) -> Uint8Array {
  xxblake3::encrypt(secret, iv, data).into()
}

#[napi]
pub fn decrypt(secret: Bin, iv: Bin, data: Bin) -> Option<Uint8Array> {
  match xxblake3::decrypt(secret, iv, data) {
    Some(i) => Some(i.into()),
    None => None,
  }
}

#[napi]
pub fn random_bytes(n: u32) -> Uint8Array {
  let r: Vec<u8> = (0..n).map(|_| rand::random::<u8>()).collect();
  r.into()
}
