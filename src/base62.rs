use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DecodeError {}

#[derive(Debug)]
pub enum EncodeError {
    InvalidUtf8(FromUtf8Error),
}

impl From<FromUtf8Error> for EncodeError {
    fn from(e: FromUtf8Error) -> Self {
        EncodeError::InvalidUtf8(e)
    }
}

const ENCODE_STRING: &[u8] =
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".as_bytes();

const DECODE: [u8; 256] = {
    let mut t = [0xFF; 256]; // 0xFF is invalid char
    let mut i = 0;
    while i < ENCODE_STRING.len() {
        t[ENCODE_STRING[i] as usize] = i as u8;
        i += 1;
    }
    t
};

pub fn encode(id: u64) -> Result<String, EncodeError> {
    let mut n = id;
    let mut ret = Vec::<u64>::new();
    while n > 0 {
        let q = n / 62;
        let r = n % 62;
        ret.push(r);
        n = q;
    }

    let mut bytes = Vec::<u8>::new();
    for v in ret {
        bytes.push(ENCODE_STRING[v as usize])
    }
    bytes.reverse();

    return Ok(String::from_utf8(bytes)?);
}

pub fn decode(encoded: &str) -> Result<u64, DecodeError> {
    let mut i = 0;
    let bytes = encoded.as_bytes();
    let mut ret: u64 = 0;
    while i < encoded.len() {
        // logic
        let exp = encoded.len() - i - 1;

        let n = DECODE[bytes[i] as usize];
        ret += 62u64.pow(exp as u32) * n as u64;

        i += 1;
    }

    return Ok(ret);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_0() {
        assert_eq!("0", encode(0).unwrap()); // TODO: fix
    }

    #[test]
    fn test_encode_simple_id() {
        let target = 3844;
        assert_eq!("100", encode(target).unwrap());
    }

    #[test]
    fn test_decode_simple_id() {
        let target = "100";
        assert_eq!(3844, decode(target).unwrap());
    }

    #[test]
    fn test_simple_roundtrip() {
        for target in 0..10000 {
            assert_eq!(target, decode(&encode(target).unwrap()).unwrap())
        }
    }
}
