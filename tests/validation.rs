use oamd::ObjectAudioMetadataPayload;

#[derive(Default)]
struct Bits {
    data: Vec<u8>,
    len: usize,
}

impl Bits {
    fn push(&mut self, width: usize, value: u32) {
        for bit in (0..width).rev() {
            if self.len.is_multiple_of(8) {
                self.data.push(0);
            }
            if value & (1 << bit) != 0 {
                self.data[self.len / 8] |= 1 << (7 - self.len % 8);
            }
            self.len += 1;
        }
    }

    fn append(&mut self, other: &Self) {
        for bit in 0..other.len {
            self.push(1, ((other.data[bit / 8] >> (7 - bit % 8)) & 1) as u32);
        }
    }
}

fn inactive_object(blocks: usize) -> Bits {
    let mut body = Bits::default();
    body.push(2, 0); // sample_offset_code
    body.push(3, blocks as u32 - 1);
    for _ in 0..blocks {
        body.push(6, 0); // block_offset_factor
        body.push(2, 0); // ramp_duration_code
    }
    body.push(1, 1); // b_reserved_data_not_present
    for _ in 0..blocks {
        body.push(1, 1); // b_object_not_active
        body.push(1, 0); // b_additional_table_data_exists
    }
    body
}

fn extended_positions() -> Bits {
    let mut body = Bits::default();
    body.push(1, 0); // b_obj_div_block
    body.push(1, 1); // b_ext_prec_pos_block; inactive objects need no position bits
    body
}

fn payload(elements: &[(u32, Bits, Option<usize>)]) -> Vec<u8> {
    let mut bits = Bits::default();
    bits.push(2, 0); // oamd_version
    bits.push(5, 0); // one object
    bits.push(1, 1); // b_dyn_object_only_program
    bits.push(1, 0); // b_lfe_present
    bits.push(1, 0); // b_alternate_object_data_present
    bits.push(4, elements.len() as u32);
    for (id, body, declared_bytes) in elements {
        let bytes = (body.len + 1).div_ceil(8);
        bits.push(4, *id);
        bits.push(4, declared_bytes.unwrap_or(bytes) as u32 - 1);
        bits.push(1, 0); // no further size group
        bits.push(1, 0); // b_discard_unknown_element
        bits.append(body);
        for _ in body.len + 1..bytes * 8 {
            bits.push(1, 0);
        }
    }
    bits.data
}

#[test]
fn replacing_object_with_fewer_blocks_rejects_stale_extended_positions() {
    let bytes = payload(&[
        (1, inactive_object(2), None),
        (5, extended_positions(), None),
        (1, inactive_object(1), None),
    ]);
    let result = ObjectAudioMetadataPayload::read(&bytes);
    assert!(
        result.is_err(),
        "incompatible extended positions were accepted"
    );
}

#[test]
fn replacing_object_with_more_blocks_rejects_stale_extended_positions() {
    let bytes = payload(&[
        (1, inactive_object(1), None),
        (5, extended_positions(), None),
        (1, inactive_object(2), None),
    ]);
    assert!(ObjectAudioMetadataPayload::read(&bytes).is_err());
}

#[test]
fn matching_repeated_object_elements_keep_extended_positions() {
    let bytes = payload(&[
        (1, inactive_object(2), None),
        (5, extended_positions(), None),
        (1, inactive_object(2), None),
    ]);
    let parsed = ObjectAudioMetadataPayload::read(&bytes).unwrap();
    assert_eq!(parsed.get_damf_pos()[0].len(), 2);
}

#[test]
fn replacing_extended_metadata_uses_the_final_object_dimensions() {
    let bytes = payload(&[
        (1, inactive_object(2), None),
        (5, extended_positions(), None),
        (1, inactive_object(1), None),
        (5, extended_positions(), None),
    ]);
    let parsed = ObjectAudioMetadataPayload::read(&bytes).unwrap();
    assert_eq!(parsed.get_damf_pos()[0].len(), 1);
}

#[test]
fn absent_extended_metadata_keeps_all_object_blocks() {
    let bytes = payload(&[(1, inactive_object(2), None)]);
    let parsed = ObjectAudioMetadataPayload::read(&bytes).unwrap();
    assert_eq!(parsed.get_damf_pos()[0].len(), 2);
}

#[test]
fn truncated_unknown_element_remains_skippable() {
    let bytes = payload(&[(0, Bits::default(), Some(16))]);
    assert!(ObjectAudioMetadataPayload::read(&bytes).is_ok());
}

#[test]
fn replacing_object_cannot_leave_stale_divergence_blocks() {
    for blocks in [1, 3] {
        let mut extended = Bits::default();
        extended.push(1, 1); // b_obj_div_block; inactive objects need no divergence bits
        extended.push(1, 0); // b_ext_prec_pos_block
        let bytes = payload(&[
            (1, inactive_object(2), None),
            (5, extended, None),
            (1, inactive_object(blocks), None),
        ]);
        assert!(ObjectAudioMetadataPayload::read(&bytes).is_err());
    }
}

#[test]
fn repeated_objects_with_empty_extended_blocks_remain_supported() {
    let mut extended = Bits::default();
    extended.push(1, 0);
    extended.push(1, 0);
    let bytes = payload(&[
        (1, inactive_object(2), None),
        (5, extended, None),
        (1, inactive_object(1), None),
    ]);
    let parsed = ObjectAudioMetadataPayload::read(&bytes).unwrap();
    assert_eq!(parsed.get_damf_pos()[0].len(), 1);
}
