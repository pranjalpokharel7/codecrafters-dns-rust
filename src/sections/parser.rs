use crate::errors::DeserializationError;

const MAX_POINTER_DEPTH: usize = 3; // Prevent infinite recursion

pub(crate) fn parse_u16_from_be_bytes(buf: &[u8]) -> Result<u16, DeserializationError> {
    if buf.len() < 2 {
        return Err(DeserializationError::UnexpectedEOF);
    }
    Ok(u16::from_be_bytes([buf[0], buf[1]]))
}

pub(crate) fn parse_u32_from_be_bytes(buf: &[u8]) -> Result<u32, DeserializationError> {
    if buf.len() < 4 {
        return Err(DeserializationError::UnexpectedEOF);
    }
    Ok(u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]))
}

// ways a label can end
// 1. with a zero octet
// 2. with a pointer
// we'll only decompress it back to the original domain name
// \0def\c0\10
// 10 -> 0001 0000 -> 16
// codecrafters.io
pub(crate) fn parse_domain_name(
    buf: &[u8],
    start: usize
) -> Result<(Vec<u8>, usize), DeserializationError> {
    parse_domain_name_impl(buf, start, 0)
}

fn parse_domain_name_impl(
    buf: &[u8],
    start: usize,
    depth: usize
) -> Result<(Vec<u8>, usize), DeserializationError> {
    if depth > MAX_POINTER_DEPTH {
        return Err(DeserializationError::CompressionTooDeep);
    }

    let mut current_pos = start;
    let mut domain_name_bytes = vec![];

    while current_pos < buf.len() {
        let length_byte = *buf.get(current_pos).ok_or(DeserializationError::UnexpectedEOF)?;

        // first condition of exit: encountered zero octet
        if length_byte == b'\x00' {
            domain_name_bytes.push(b'\x00');
            current_pos += 1; // move past null byte
            break;
        }

        let is_compressed_pointer = ((length_byte & 0xc0) >> 6) == 0b11;
        if is_compressed_pointer {
            let target_offset = (parse_u16_from_be_bytes(&buf[current_pos..])? & 0x3fff) as usize;
            if target_offset >= current_pos {
                return Err(DeserializationError::InvalidCompressionPointer);
            }

            let (label, _) = parse_domain_name_impl(&buf, target_offset, depth + 1)?;
            domain_name_bytes.extend(label);
            current_pos += 2; // compressed pointers are 2 bytes in size
            break;
        } else {
            // length byte is also included in label so we add 1
            let label_end = current_pos + (length_byte as usize) + 1;
            if label_end > buf.len() {
                return Err(DeserializationError::UnexpectedEOF);
            }

            domain_name_bytes.extend(&buf[current_pos..label_end]);
            current_pos = label_end;
        }
    }

    Ok((domain_name_bytes, current_pos))
}
