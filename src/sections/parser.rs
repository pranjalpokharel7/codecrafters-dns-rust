use crate::errors::DeserializationError;

pub(crate) fn parse_u16_from_be_bytes(buf: &[u8]) -> Result<u16, DeserializationError> {
    Ok(
        u16::from_be_bytes([
            *buf.get(0).ok_or(DeserializationError::UnexpectedEOF)?,
            *buf.get(1).ok_or(DeserializationError::UnexpectedEOF)?,
        ])
    )
}

pub(crate) fn parse_u32_from_be_bytes(buf: &[u8]) -> Result<u32, DeserializationError> {
    Ok(
        u32::from_be_bytes([
            *buf.get(0).ok_or(DeserializationError::UnexpectedEOF)?,
            *buf.get(1).ok_or(DeserializationError::UnexpectedEOF)?,
            *buf.get(2).ok_or(DeserializationError::UnexpectedEOF)?,
            *buf.get(3).ok_or(DeserializationError::UnexpectedEOF)?,
        ])
    )
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
    let mut pos = start;
    let mut name = vec![];

    while pos < buf.len() {
        let length_byte = buf[pos];

        // first condition of exit: encountered zero octet
        if length_byte == b'\x00' {
            name.push(b'\x00');
            pos += 1; // move past null byte
            break;
        }

        let is_compressed_pointer = ((length_byte & 0xc0) >> 6) == 0b11;
        if is_compressed_pointer {
            let ptr = (u16::from_be_bytes([buf[pos], buf[pos + 1]]) & 0x3fff) as usize;
            let (label, _) = parse_domain_name(&buf, ptr)?;
            name.extend(label);
            pos += 2; // move past null byte
            break; // second condition of exit: encountered pointer
        } else {
            // length byte is also included in label so we add 1
            let end = pos + (length_byte as usize) + 1;
            name.extend(&buf[pos..end]);
            pos = end;
        }
    }

    Ok((name, pos))
}
