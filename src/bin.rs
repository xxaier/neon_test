use napi::{
  bindgen_prelude::{Buffer, FromNapiValue, TypeName, ValidateNapiValue},
  sys::{napi_env, napi_value},
  Either,
};

pub type StringOrBuffer = Either<String, Buffer>;

pub struct Bin(StringOrBuffer);

impl AsRef<[u8]> for Bin {
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl FromNapiValue for Bin {
  unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
    Ok(Self(StringOrBuffer::from_napi_value(env, napi_val)?))
  }
}

impl TypeName for Bin {
  fn type_name() -> &'static str {
    "String/Buffer/Uint8Array"
  }

  fn value_type() -> napi::ValueType {
    StringOrBuffer::value_type()
  }
}

impl ValidateNapiValue for Bin {
  unsafe fn validate(env: napi_env, napi_val: napi_value) -> napi::Result<napi_value> {
    unsafe { StringOrBuffer::validate(env, napi_val) }
  }
}

pub type EitherBinVec = Either<Bin, Vec<Bin>>;

pub struct Bins(EitherBinVec);

impl FromNapiValue for Bins {
  unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
    Ok(Self(EitherBinVec::from_napi_value(env, napi_val)?))
  }
}

impl IntoIterator for Bins {
  type Item = Bin;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}
