use std::convert::TryInto;

use anyhow::anyhow;
use wasmer::{AsStoreMut, Function, Memory, Store, Value};

pub fn lower_string(
    store: &mut Store,
    value: &str,
    __new: &Function,
    __pin: &Function,
    memory: &Memory,
) -> anyhow::Result<u32> {
    let length: i32 = value.len().try_into()?;

    let result = __new.call(
        &mut store.as_store_mut(),
        &[Value::I32(length << 1), Value::I32(2)],
    )?;

    let pointer = result
        .get(0)
        .ok_or(anyhow!("can't get new string pointer"))?
        .i32()
        .ok_or(anyhow!("can't get new string pointer"))?;

    let utf16: Vec<u16> = value.encode_utf16().collect();
    let utf16_to_u8: &[u8] = bytemuck::try_cast_slice(&utf16.as_slice()).expect("qaq");

    let view = memory.view(&store);

    view.write(pointer as u64, utf16_to_u8).unwrap();
    __pin.call(&mut store.as_store_mut(), &[Value::I32(pointer)])?;

    Ok(pointer as u32)
}
