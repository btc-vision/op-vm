use crate::common::MockInstanceWrapper;
use op_vm::domain::runner::import_functions::common::DataSliceWriter;
use wasmer::AsStoreMut;

#[test]
fn test_write_data_no_padding_needed() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        0,
        8,
        10,
    );

    assert!(result.is_ok());
    instance.assert_memory_equals(10, &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(instance.write_call_count(), 1);
}

#[test]
fn test_zero_length_request() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3, 4];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        2,
        0,
        40,
    );

    assert!(result.is_ok());
    assert_eq!(instance.write_call_count(), 0);
}

#[test]
fn test_empty_data_array() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        0,
        5,
        60,
    );

    assert!(result.is_ok());
    assert_eq!(instance.write_call_count(), 1);
    instance.assert_memory_equals(60, &[0, 0, 0, 0, 0]);
}

#[test]
fn test_offset_equals_data_length() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3, 4];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        4,
        3,
        50,
    );

    assert!(result.is_ok());

    let writes = instance.get_write_calls();
    assert_eq!(writes.len(), 1);
    assert_eq!(writes[0].data.len(), 3);
}

#[test]
fn test_large_offset() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        100,
        5,
        0,
    );

    assert!(result.is_ok());
    let writes = instance.get_write_calls();
    assert_eq!(writes.len(), 1);

    assert_eq!(
        writes[0].data.len(),
        5,
        "Should write exactly 'length' bytes when offset is beyond data"
    );
    assert_eq!(writes[0].offset, 0, "Padding should start at result_ptr");
}

#[test]
fn test_u32_max_values() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        u32::MAX - 10,
        5,
        0,
    );

    assert!(result.is_ok());
}

#[test]
fn test_padding_should_immediately_follow_data() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        6,
        4,
        20,
    );

    assert!(result.is_ok());

    instance.assert_memory_equals(20, &[7, 8, 0, 0]);
}

#[test]
fn test_no_memory_gap_after_data() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        6,
        4,
        20,
    );

    assert!(result.is_ok());

    let memory = instance.get_memory_slice(22, 6);
    assert!(
        memory.iter().any(|&b| b != 0xFF),
        "Memory gap detected at offset 22! Memory should have been overwritten with padding"
    );
}

#[test]
fn test_padding_at_correct_offset() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3, 4];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        10,
        5,
        30,
    );

    assert!(result.is_ok());

    let writes = instance.get_write_calls();
    assert_eq!(writes.len(), 1, "Should have one padding write");
    assert_eq!(
        writes[0].offset, 30,
        "Padding should be written at result_ptr (30), not result_ptr + data.len (34)"
    );
}

#[test]
fn test_correct_padding_size() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3, 4];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        10,
        5,
        30,
    );

    assert!(result.is_ok());

    let writes = instance.get_write_calls();
    assert_eq!(
        writes[0].data.len(),
        5,
        "Should write 'length' bytes of padding, not (offset + length - data.len)"
    );
}

#[test]
fn test_security_no_data_leak() {
    let mut instance = MockInstanceWrapper::new(200, 10000);
    let mut store = crate::common::create_mock_store();

    let sensitive_data = b"SECRET_PASSWORD_123";
    instance.set_memory(22, sensitive_data);

    let data = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11, 0x22];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        6,
        4,
        20,
    );

    assert!(result.is_ok());

    let memory_at_22 = instance.get_memory_slice(22, 2);
    assert_eq!(
        memory_at_22,
        vec![0, 0],
        "Memory at offset 22-23 should be zeroed (padding), not contain old data"
    );
}

#[test]
fn test_write_memory_error_handling() {
    let instance = MockInstanceWrapper::new(10, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3, 4];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        0,
        4,
        8,
    );

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Error writing data to memory"));
}

#[test]
fn test_all_code_paths() {
    let mut instance = MockInstanceWrapper::new(1000, 10000);
    let mut store = crate::common::create_mock_store();

    let test_cases = vec![
        (vec![], 0, 0, 0, "empty data, zero length"),
        (vec![1], 0, 1, 100, "single byte, exact match"),
        (vec![1, 2], 1, 1, 200, "partial data, no padding"),
        (vec![1, 2, 3], 0, 5, 300, "data with padding"),
        (
            vec![1, 2, 3, 4],
            2,
            5,
            400,
            "offset in middle, with padding",
        ),
        (vec![1, 2], 10, 5, 500, "offset beyond data"),
        (vec![1, 2, 3, 4, 5], 5, 0, 600, "zero length at end"),
    ];

    for (data, offset, length, result_ptr, description) in test_cases {
        instance.clear_calls();

        let result = DataSliceWriter::write_data_and_padding_to_memory(
            &mut store.as_store_mut(),
            &instance,
            &data,
            offset,
            length,
            result_ptr,
        );

        assert!(result.is_ok(), "Failed for case: {}", description);

        let writes = instance.get_write_calls();
        println!("Test '{}': {} writes", description, writes.len());
    }
}

#[test]
fn test_slice_in_bounds_coverage() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();

    let data = vec![1, 2, 3];
    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        10,
        5,
        0,
    );
    assert!(result.is_ok());

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        2,
        10,
        10,
    );
    assert!(result.is_ok());

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        1,
        1,
        20,
    );
    assert!(result.is_ok());
}

#[test]
fn test_offset_plus_length_overflow() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3, 4, 5];

    let offset = u32::MAX - 2;
    let length = 5;

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        offset,
        length,
        0,
    );

    assert!(result.is_ok());

    // Should only write padding since offset is way beyond data
    instance.assert_memory_equals(0, &vec![0; 5]);
}

#[test]
fn test_offset_plus_length_overflow_behavior() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3, 4, 5];

    // When offset + length overflows
    let offset = u32::MAX - 2; // 4294967293
    let length = 5;
    // offset + length would overflow to 2

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        offset,
        length,
        10,
    );

    assert!(result.is_ok());

    let writes = instance.get_write_calls();

    // So data_written = 0, and we write length 5 bytes of padding
    assert_eq!(writes.len(), 1);
    assert_eq!(writes[0].data.len(), 5);
    assert_eq!(writes[0].offset, 10);
    assert!(writes[0].data.iter().all(|&b| b == 0));
}

#[test]
fn test_data_written_determines_padding_offset() {
    let instance = MockInstanceWrapper::new(100, 10000);
    let mut store = crate::common::create_mock_store();
    let data = vec![1, 2, 3, 4, 5];

    let result = DataSliceWriter::write_data_and_padding_to_memory(
        &mut store.as_store_mut(),
        &instance,
        &data,
        1,
        5,
        20,
    );

    assert!(result.is_ok());

    let writes = instance.get_write_calls();
    assert_eq!(writes.len(), 2, "Should have two writes: data and padding");

    assert_eq!(writes[0].offset, 20);
    assert_eq!(writes[0].data, vec![2, 3, 4, 5]);

    assert_eq!(writes[1].offset, 24);
    assert_eq!(writes[1].data, vec![0]);
}
