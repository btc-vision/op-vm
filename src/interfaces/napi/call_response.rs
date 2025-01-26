use napi::bindgen_prelude::BigInt;

pub struct CallResponseRaw {
    pub result: i32,
    pub gas_used: u64,
}

#[napi(object)]
pub struct CallResponse {
    pub result: i32,
    pub gas_used: BigInt,
}

/*pub struct CallResponse {
    //#[napi(ts_type = "number[]")]
    pub result: JsNumber,
    pub gas_used: BigInt,
}*/
