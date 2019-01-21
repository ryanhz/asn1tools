//! This file was generated by asn1tools version 0.145.2 Sun Jan 20 10:31:38 2019.

struct Encoder<'a> {
    buf: &'a mut [u8],
    size: usize,
    pos: usize,
    error: Option<&'static str>
}

struct Decoder<'a> {
    buf: &'a[u8],
    size: usize,
    pos: usize,
    error: Option<&'static str>
}

impl<'a> Encoder<'a> {

    fn new(dst: &'a mut [u8]) -> Encoder {
        Encoder {
            size: 8 * dst.len(),
            buf: dst,
            pos: 0,
            error: None
        }
    }

    fn get_result(&self) -> Result<usize, &'static str>
    {
        if self.error.is_none() {
            return Ok((self.pos + 7) / 8);
        } else {
            return Err(self.error.unwrap());
        }
    }

    fn abort(&mut self, error: &'static str)
    {
        if self.error.is_none() {
            self.error = Some(error);
        }
    }

    fn alloc(&mut self, size: usize) -> Result<usize, ()>
    {
        if self.pos + size <= self.size {
            let pos = self.pos;
            self.pos += size;
            Ok(pos)
        } else {
            self.abort("Out of memory.");
            Err(())
        }
    }

    fn append_bit(&mut self, value: u8)
    {
        if let Ok(pos) = self.alloc(1) {
            if pos % 8 == 0 {
                self.buf[pos / 8] = 0;
            }

            self.buf[pos / 8] |= value << (7 - (pos % 8));
        }
    }

    fn append_bytes(&mut self, buf: &[u8])
    {
        if let Ok(pos) = self.alloc(8 * buf.len()) {
            let byte_pos = pos / 8;
            let pos_in_byte = pos % 8;

            if pos_in_byte == 0 {
                self.buf.get_mut(byte_pos..byte_pos + buf.len())
                    .unwrap()
                    .copy_from_slice(buf.get(0..buf.len()).unwrap());
            } else {
                for i in 0..buf.len() {
                    self.buf[byte_pos + i] |= buf[i] >> pos_in_byte;
                    self.buf[byte_pos + i + 1] = buf[i] << (8 - pos_in_byte);
                }
            }
        }
    }

    fn append_u8(&mut self, value: u8)
    {
        self.append_bytes(&[value]);
    }

    fn append_u16(&mut self, value: u16)
    {
        self.append_bytes(&value.to_be_bytes());
    }

    fn append_u32(&mut self, value: u32)
    {
        self.append_bytes(&value.to_be_bytes());
    }

    fn append_u64(&mut self, value: u64)
    {
        self.append_bytes(&value.to_be_bytes());
    }

    fn append_i8(&mut self, value: i8)
    {
        self.append_u8((value as u8).wrapping_add(128));
    }

    fn append_i16(&mut self, value: i16)
    {
        self.append_u16((value as u16).wrapping_add(32768));
    }

    fn append_i32(&mut self, value: i32)
    {
        self.append_u32((value as u32).wrapping_add(2147483648));
    }

    fn append_i64(&mut self, value: i64)
    {
        self.append_u64((value as u64).wrapping_add(9223372036854775808));
    }

    fn append_bool(&mut self, value: bool)
    {
        self.append_bit(value as u8);
    }

    fn append_non_negative_binary_integer(&mut self, value: u64, size: usize)
    {
        for i in 0..size {
            self.append_bit((value >> (size - i - 1)) as u8 & 1);
        }
    }
}

impl<'a> Decoder<'a> {

    fn new(src: &'a[u8]) -> Decoder {
        Decoder {
            buf: src,
            size: 8 * src.len(),
            pos: 0,
            error: None
        }
    }

    fn get_result(&self) -> Result<usize, &'static str>
    {
        if self.error.is_none() {
            Ok((self.pos + 7) / 8)
        } else {
            Err(self.error.unwrap())
        }
    }

    fn abort(&mut self, error: &'static str)
    {
        if self.error.is_none() {
            self.error = Some(error);
        }
    }

    fn free(&mut self, size: usize) -> Result<usize, ()>
    {
        if self.pos + size <= self.size {
            let pos = self.pos;
            self.pos += size;
            Ok(pos)
        } else {
            self.abort("Out of data.");
            Err(())
        }
    }

    fn read_bit(&mut self) -> u8
    {
        if let Ok(pos) = self.free(1) {
            (self.buf[pos / 8] >> (7 - (pos % 8))) & 1
        } else {
            0
        }
    }

    fn read_bytes(&mut self, buf: &mut [u8])
    {
        if let Ok(pos) = self.free(8 * buf.len()) {
            let byte_pos = pos / 8;
            let pos_in_byte = pos % 8;

            if pos_in_byte == 0 {
                buf.copy_from_slice(
                    self.buf.get(byte_pos..byte_pos + buf.len()).unwrap());
            } else {
                for i in 0..buf.len() {
                    buf[i] = self.buf[byte_pos + i] << pos_in_byte;
                    buf[i] |= self.buf[byte_pos + i + 1] >> (8 - pos_in_byte);
                }
            }
        }
    }

    fn read_u8(&mut self) -> u8
    {
        let mut buf = [0; 1];

        self.read_bytes(&mut buf);

        u8::from_be_bytes(buf)
    }

    fn read_u16(&mut self) -> u16
    {
        let mut buf = [0; 2];

        self.read_bytes(&mut buf);

        u16::from_be_bytes(buf)
    }

    fn read_u32(&mut self) -> u32
    {
        let mut buf = [0; 4];

        self.read_bytes(&mut buf);

        u32::from_be_bytes(buf)
    }

    fn read_u64(&mut self) -> u64
    {
        let mut buf = [0; 8];

        self.read_bytes(&mut buf);

        u64::from_be_bytes(buf)
    }

    fn read_i8(&mut self) -> i8
    {
        self.read_u8().wrapping_sub(128) as i8
    }

    fn read_i16(&mut self) -> i16
    {
        self.read_u16().wrapping_sub(32768) as i16
    }

    fn read_i32(&mut self) -> i32
    {
        self.read_u32().wrapping_sub(2147483648) as i32
    }

    fn read_i64(&mut self) -> i64
    {
        self.read_u64().wrapping_sub(9223372036854775808) as i64
    }

    fn read_bool(&mut self) -> bool
    {
        self.read_bit() != 0
    }

    fn read_non_negative_binary_integer(&mut self, size: usize) -> u64
    {
        let mut value: u64 = 0;

        for _ in 0..size {
            value <<= 1;
            value |= self.read_bit() as u64;
        }

        value
    }
}

/// Module RustSource.
pub mod rust_source {

    /// Type A.
    pub mod a {

        use super::super::{Encoder, Decoder};

        #[derive(Debug, Default, PartialEq)]
        pub struct AJ {
            pub buf: [u8; 11]
        }

        #[derive(Debug, Default, PartialEq)]
        pub struct A {
            pub a: i8,
            pub b: i16,
            pub c: i32,
            pub d: i64,
            pub e: u8,
            pub f: u16,
            pub g: u32,
            pub h: u64,
            pub i: bool,
            pub j: AJ
        }

        impl A {

            pub fn to_bytes(&mut self, mut dst: &mut [u8]) -> Result<usize, &'static str>
            {
                let mut encoder = Encoder::new(&mut dst);

                self.to_bytes_inner(&mut encoder);

                encoder.get_result()
            }

            pub fn from_bytes(&mut self, src: &[u8]) -> Result<usize, &'static str>
            {
                let mut decoder = Decoder::new(&src);

                self.from_bytes_inner(&mut decoder);

                decoder.get_result()
            }

            fn to_bytes_inner(&mut self, encoder: &mut Encoder)
            {
                encoder.append_i8(self.a);
                encoder.append_i16(self.b);
                encoder.append_i32(self.c);
                encoder.append_i64(self.d);
                encoder.append_u8(self.e);
                encoder.append_u16(self.f);
                encoder.append_u32(self.g);
                encoder.append_u64(self.h);
                encoder.append_bool(self.i);
                encoder.append_bytes(&self.j.buf);
            }

            fn from_bytes_inner(&mut self, decoder: &mut Decoder)
            {
                self.a = decoder.read_i8();
                self.b = decoder.read_i16();
                self.c = decoder.read_i32();
                self.d = decoder.read_i64();
                self.e = decoder.read_u8();
                self.f = decoder.read_u16();
                self.g = decoder.read_u32();
                self.h = decoder.read_u64();
                self.i = decoder.read_bool();
                decoder.read_bytes(&mut self.j.buf);
            }
        }
    }

    /// Type D.
    pub mod d {

        use super::super::{Encoder, Decoder};

        #[derive(Debug, PartialEq, Copy, Clone)]
        pub enum DElemGH {
            I,
            J,
            K
        }

        impl Default for DElemGH {
            fn default() -> Self { DElemGH::I }
        }

        #[derive(Debug, PartialEq, Copy, Clone)]
        pub enum DElemAB {
            C(u8),
            D(bool)
        }

        impl Default for DElemAB {
            fn default() -> Self { DElemAB::C(0) }
        }

        #[derive(Debug, Default, PartialEq, Copy, Clone)]
        pub struct DElemAE {
            pub length: u8
        }

        #[derive(Debug, Default, PartialEq, Copy, Clone)]
        pub struct DElemA {
            pub b: DElemAB,
            pub e: DElemAE
        }

        #[derive(Debug, Default, PartialEq, Copy, Clone)]
        pub struct DElemGL {
            pub length: u8,
            pub buf: [u8; 2]
        }

        #[derive(Debug, Default, PartialEq, Copy, Clone)]
        pub struct DElemG {
            pub h: DElemGH,
            pub l: DElemGL
        }

        #[derive(Debug, Default, PartialEq, Copy, Clone)]
        pub struct DElemMPQ {
            pub buf: [u8; 5]
        }

        #[derive(Debug, Default, PartialEq, Copy, Clone)]
        pub struct DElemMP {
            pub q: DElemMPQ,
            pub is_r_present: bool,
            pub r: bool
        }

        #[derive(Debug, Default, PartialEq, Copy, Clone)]
        pub struct DElemM {
            pub is_n_present: bool,
            pub n: bool,
            pub o: i8,
            pub is_p_present: bool,
            pub p: DElemMP
        }

        #[derive(Debug, Default, PartialEq, Copy, Clone)]
        pub struct DElem {
            pub a: DElemA,
            pub g: DElemG,
            pub m: DElemM
        }

        #[derive(Debug, Default, PartialEq, Copy, Clone)]
        pub struct D {
            pub length: u8,
            pub elements: [DElem; 10]
        }

        impl D {

            pub fn to_bytes(&mut self, mut dst: &mut [u8]) -> Result<usize, &'static str>
            {
                let mut encoder = Encoder::new(&mut dst);

                self.to_bytes_inner(&mut encoder);

                encoder.get_result()
            }

            pub fn from_bytes(&mut self, src: &[u8]) -> Result<usize, &'static str>
            {
                let mut decoder = Decoder::new(&src);

                self.from_bytes_inner(&mut decoder);

                decoder.get_result()
            }

            fn to_bytes_inner(&mut self, encoder: &mut Encoder)
            {
                encoder.append_non_negative_binary_integer(
                    self.length as u64 - 1,
                    4);

                for i in 0..self.length as usize {
                    match self.elements[i].a.b {
                        DElemAB::C(value) => {
                            encoder.append_non_negative_binary_integer(0, 1);
                            encoder.append_non_negative_binary_integer(
                                value as u64 - 0,
                                1);
                        },

                        DElemAB::D(value) => {
                            encoder.append_non_negative_binary_integer(1, 1);
                            encoder.append_bool(value);
                        }
                    }

                    encoder.append_non_negative_binary_integer(
                        self.elements[i].a.e.length as u64 - 3,
                        1);
                    encoder.append_non_negative_binary_integer(
                        self.elements[i].g.h as u64,
                        2);
                    encoder.append_non_negative_binary_integer(
                        self.elements[i].g.l.length as u64 - 1,
                        1);
                    encoder.append_bytes(
                        &mut self.elements[i].g.l.buf[0..self.elements[i].g.l.length as usize]);
                    encoder.append_bool(self.elements[i].m.is_n_present);
                    encoder.append_bool(self.elements[i].m.o != 3);
                    encoder.append_bool(self.elements[i].m.is_p_present);

                    if self.elements[i].m.is_n_present {
                        encoder.append_bool(self.elements[i].m.n);
                    }

                    if self.elements[i].m.o != 3 {
                        encoder.append_non_negative_binary_integer(
                            (self.elements[i].m.o - -2) as u64,
                            3);
                    }

                    if self.elements[i].m.is_p_present {
                        encoder.append_bool(self.elements[i].m.p.is_r_present);
                        encoder.append_bytes(&self.elements[i].m.p.q.buf);

                        if self.elements[i].m.p.is_r_present {
                            encoder.append_bool(self.elements[i].m.p.r);
                        }
                    }
                }
            }

            fn from_bytes_inner(&mut self, decoder: &mut Decoder)
            {
                self.length = decoder.read_non_negative_binary_integer(4) as u8;
                self.length += 1;

                if self.length > 10 {
                    decoder.abort("Bad length.");

                    return;
                }

                for i in 0..self.length as usize {
                    match decoder.read_non_negative_binary_integer(1) {
                        0 => {
                            self.elements[i].a.b =
                                DElemAB::C(decoder.read_non_negative_binary_integer(1) as u8 + 0);
                        },

                        1 => {
                            self.elements[i].a.b =
                                DElemAB::D(decoder.read_bool());
                        },

                        _ => {
                            decoder.abort("Bad choice.");

                            return;
                        }
                    }

                    self.elements[i].a.e.length =
                        decoder.read_non_negative_binary_integer(1) as u8;
                    self.elements[i].a.e.length += 3;
                    self.elements[i].g.h =
                        match decoder.read_non_negative_binary_integer(2) {
                            0 => DElemGH::I,

                            1 => DElemGH::J,

                            2 => DElemGH::K,

                            _ => {
                                decoder.abort("Bad choice.");

                                return;
                            }
                        };
                    self.elements[i].g.l.length =
                        decoder.read_non_negative_binary_integer(1) as u8;
                    self.elements[i].g.l.length += 1;
                    decoder.read_bytes(
                        &mut self.elements[i].g.l.buf[0..self.elements[i].g.l.length as usize]);
                    self.elements[i].m.is_n_present = decoder.read_bool();
                    let is_present = decoder.read_bool();
                    self.elements[i].m.is_p_present = decoder.read_bool();

                    if self.elements[i].m.is_n_present {
                        self.elements[i].m.n = decoder.read_bool();
                    }

                    if is_present {
                        self.elements[i].m.o =
                            decoder.read_non_negative_binary_integer(3) as i8;
                        self.elements[i].m.o += -2;
                    } else {
                        self.elements[i].m.o = 3;
                    }

                    if self.elements[i].m.is_p_present {
                        self.elements[i].m.p.is_r_present = decoder.read_bool();
                        decoder.read_bytes(
                            &mut self.elements[i].m.p.q.buf);

                        if self.elements[i].m.p.is_r_present {
                            self.elements[i].m.p.r = decoder.read_bool();
                        }
                    }
                }
            }
        }
    }
}
