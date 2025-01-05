use std::io::Read;

#[derive(Debug, Clone, Copy)]
struct Rotation(u8);

impl Rotation {
    fn new(rot: u8) -> Self {
        Self(rot % 26)
    }
}

struct RotDecoder<R: Read> {
    input: R,
    rotation: Rotation,
}

impl<R: Read> RotDecoder<R> {
    fn new(input: R, rotation: u8) -> Self {
        Self {
            input,
            rotation: Rotation::new(rotation),
        }
    }
}

fn rot13(byte: u8, Rotation(rot): Rotation) -> u8 {
    match byte {
        b'A'..=b'Z' => (byte - b'A' + rot) % 26 + b'A',
        b'a'..=b'z' => (byte - b'a' + rot) % 26 + b'a',
        _ => byte,
    }
}

impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.input.read(buf)?;
        for byte in &mut buf[..bytes_read] {
            *byte = rot13(*byte, self.rotation);
        }
        Ok(bytes_read)
    }
}

fn main() {
    let input = "Gb trg gb gur bgure fvqr!";
    let mut rot = RotDecoder::new(input.as_bytes(), 13);
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);

    let input = "To get to the other side!";
    let mut rot = RotDecoder::new(input.as_bytes(), 13);
    let mut rot2 = RotDecoder::new(rot, 13);
    let mut result = String::new();
    rot2.read_to_string(&mut result).unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let input = "Gb trg gb gur bgure fvqr!";
        let mut rot = RotDecoder::new(input.as_bytes(), 13);
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]>::new(input.as_ref(), 13);
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}