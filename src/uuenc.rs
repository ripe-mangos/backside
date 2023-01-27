
use bytes::Bytes;
use bitvec::prelude::*;
use std::io::Result;

pub fn u6(bs: &BitSlice) -> u8 {
    match bs {
        _ => unimplemented!()
    }
}

pub fn uuencode(b: Bytes) -> Result<Vec<u8>> {
    let mut uu: Vec<u8> = Vec::new();
    let mut bits: BitVec = BitVec::new();
    bits.extend_from_bitslice(b.as_bits::<Lsb0>());

    let mut chunks = bits.chunks(6);
    loop {
        let ch = chunks.next();
        if ch.is_none() {
            break;
        }

        //let ch = ch.unwrap();
        let c = ch.unwrap().load::<u8>() + 33;

        uu.push(c);
    }

    Ok(uu)
}

pub unsafe fn uudecode(s: &str) {
    let mut chars = s.chars();
    loop {
        let c = chars.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
    }
}