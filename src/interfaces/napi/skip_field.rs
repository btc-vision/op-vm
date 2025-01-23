use napi::{
    bindgen_prelude::{FromNapiValue, ToNapiValue, TypeName, ValidateNapiValue, ValueType},
    sys, Error, Result, Status,
};

/// A zero-sized type that appears as `undefined` in JS,
/// effectively skipping real exposure.
#[derive(Debug)]
pub struct SkipField;

impl TypeName for SkipField {
    fn type_name() -> &'static str {
        "SkipField"
    }
    fn value_type() -> ValueType {
        // We'll pretend it's `Undefined`.
        ValueType::Undefined
    }
}

impl ValidateNapiValue for SkipField {}

impl FromNapiValue for SkipField {
    /// If JavaScript tries to set this field, we can error out or just allow it.
    unsafe fn from_napi_value(_env: sys::napi_env, _value: sys::napi_value) -> Result<Self> {
        // We'll just produce SkipField but it's basically meaningless
        Ok(SkipField)
    }
}

impl ToNapiValue for SkipField {
    unsafe fn to_napi_value(env: sys::napi_env, _val: Self) -> Result<sys::napi_value> {
        // Return `undefined` so the field is invisible to typical usage
        let mut raw_undefined = std::ptr::null_mut();
        let status = sys::napi_get_undefined(env, &mut raw_undefined);
        if status == sys::Status::napi_ok {
            Ok(raw_undefined)
        } else {
            Err(Error::new(
                Status::GenericFailure,
                "Failed to get `undefined`",
            ))
        }
    }
}
