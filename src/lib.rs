//! Adds methods to read/write binary numbers to the [`Read`] and [`Write`] traits.

use std::io::{Read, Write};

#[cfg(feature = "impl")]
macro_rules! writer_impl {
    ($lename:tt, $bename:tt, $t:ty) => {
        #[inline(always)]
        fn $lename(&mut self, v: $t) -> std::io::Result<usize> {
            self.write(&v.to_le_bytes())
        }

        #[inline(always)]
        fn $bename(&mut self, v: $t) -> std::io::Result<usize> {
            self.write(&v.to_be_bytes())
        }
    };
}

pub trait WriterTypes {
    /// Write a [`u8`] value to the sink.
    fn write_u8(&mut self, v: u8) -> std::io::Result<usize>;

    /// Write a [`i16`] value to the sink in little endian.
    fn write_i16_le(&mut self, v: i16) -> std::io::Result<usize>;
    /// Write a [`i16`] value to the sink in big endian.
    fn write_i16_be(&mut self, v: i16) -> std::io::Result<usize>;
    /// Write a [`i32`] value to the sink in little endian.
    fn write_i32_le(&mut self, v: i32) -> std::io::Result<usize>;
    /// Write a [`i32`] value to the sink in big endian.
    fn write_i32_be(&mut self, v: i32) -> std::io::Result<usize>;
    /// Write a [`i64`] value to the sink in little endian.
    fn write_i64_le(&mut self, v: i64) -> std::io::Result<usize>;
    /// Write a [`i64`] value to the sink in big endian.
    fn write_i64_be(&mut self, v: i64) -> std::io::Result<usize>;
    /// Write a [`i128`] value to the sink in little endian.
    fn write_i128_le(&mut self, v: i128) -> std::io::Result<usize>;
    /// Write a [`i128`] value to the sink in big endian.
    fn write_i128_be(&mut self, v: i128) -> std::io::Result<usize>;

    /// Write a [`u16`] value to the sink in little endian.
    fn write_u16_le(&mut self, v: u16) -> std::io::Result<usize>;
    /// Write a [`u16`] value to the sink in big endian.
    fn write_u16_be(&mut self, v: u16) -> std::io::Result<usize>;
    /// Write a [`u32`] value to the sink in little endian.
    fn write_u32_le(&mut self, v: u32) -> std::io::Result<usize>;
    /// Write a [`u32`] value to the sink in big endian.
    fn write_u32_be(&mut self, v: u32) -> std::io::Result<usize>;
    /// Write a [`u64`] value to the sink in little endian.
    fn write_u64_le(&mut self, v: u64) -> std::io::Result<usize>;
    /// Write a [`u64`] value to the sink in big endian.
    fn write_u64_be(&mut self, v: u64) -> std::io::Result<usize>;
    /// Write a [`u128`] value to the sink in little endian.
    fn write_u128_le(&mut self, v: u128) -> std::io::Result<usize>;
    /// Write a [`u128`] value to the sink in big endian.
    fn write_u128_be(&mut self, v: u128) -> std::io::Result<usize>;
}

#[cfg(feature = "impl")]
impl<W: Write> WriterTypes for W {
    #[inline(always)]
    fn write_u8(&mut self, v: u8) -> std::io::Result<usize> {
        self.write(&[v])
    }

    writer_impl! {write_i16_le, write_i16_be, i16}
    writer_impl! {write_i32_le, write_i32_be, i32}
    writer_impl! {write_i64_le, write_i64_be, i64}
    writer_impl! {write_i128_le, write_i128_be, i128}

    writer_impl! {write_u16_le, write_u16_be, u16}
    writer_impl! {write_u32_le, write_u32_be, u32}
    writer_impl! {write_u64_le, write_u64_be, u64}
    writer_impl! {write_u128_le, write_u128_be, u128}
}

pub trait ReaderTypes {
    /// Read a [`u8`] value from the sink.
    fn read_u8(&mut self) -> std::io::Result<u8>;

    /// Read a [`i16`] value from the sink in little endian.
    fn read_i16_le(&mut self) -> std::io::Result<i16>;
    /// Read a [`i16`] value from the sink in big endian.
    fn read_i16_be(&mut self) -> std::io::Result<i16>;
    /// Read a [`i32`] value from the sink in little endian.
    fn read_i32_le(&mut self) -> std::io::Result<i32>;
    /// Read a [`i32`] value from the sink in big endian.
    fn read_i32_be(&mut self) -> std::io::Result<i32>;
    /// Read a [`i64`] value from the sink in little endian.
    fn read_i64_le(&mut self) -> std::io::Result<i64>;
    /// Read a [`i64`] value from the sink in big endian.
    fn read_i64_be(&mut self) -> std::io::Result<i64>;
    /// Read a [`i128`] value from the sink in little endian.
    fn read_i128_le(&mut self) -> std::io::Result<i128>;
    /// Read a [`i128`] value from the sink in big endian.
    fn read_i128_be(&mut self) -> std::io::Result<i128>;

    /// Read a [`u16`] value from the sink in little endian.
    fn read_u16_le(&mut self) -> std::io::Result<u16>;
    /// Read a [`u16`] value from the sink in big endian.
    fn read_u16_be(&mut self) -> std::io::Result<u16>;
    /// Read a [`u32`] value from the sink in little endian.
    fn read_u32_le(&mut self) -> std::io::Result<u32>;
    /// Read a [`u32`] value from the sink in big endian.
    fn read_u32_be(&mut self) -> std::io::Result<u32>;
    /// Read a [`u64`] value from the sink in little endian.
    fn read_u64_le(&mut self) -> std::io::Result<u64>;
    /// Read a [`u64`] value from the sink in big endian.
    fn read_u64_be(&mut self) -> std::io::Result<u64>;
    /// Read a [`u128`] value from the sink in little endian.
    fn read_u128_le(&mut self) -> std::io::Result<u128>;
    /// Read a [`u128`] value from the sink in big endian.
    fn read_u128_be(&mut self) -> std::io::Result<u128>;
}

#[cfg(feature = "impl")]
macro_rules! reader_impl {
    ($lename:tt, $bename:tt, $t:ty) => {
        #[inline(always)]
        fn $lename(&mut self) -> std::io::Result<$t> {
            let mut buf = [0; std::mem::size_of::<$t>()];
            self.read(&mut buf)?;
            Ok(<$t>::from_le_bytes(buf))
        }

        #[inline(always)]
        fn $bename(&mut self) -> std::io::Result<$t> {
            let mut buf = [0; std::mem::size_of::<$t>()];
            self.read(&mut buf)?;
            Ok(<$t>::from_be_bytes(buf))
        }
    };
}

#[cfg(feature = "impl")]
impl<R: Read> ReaderTypes for R {
    #[inline(always)]
    fn read_u8(&mut self) -> std::io::Result<u8> {
        let mut buf = [0; 1];
        self.read(&mut buf)?;
        Ok(buf[0])
    }

    reader_impl! {read_i16_le, read_i16_be, i16}
    reader_impl! {read_i32_le, read_i32_be, i32}
    reader_impl! {read_i64_le, read_i64_be, i64}
    reader_impl! {read_i128_le, read_i128_be, i128}

    reader_impl! {read_u16_le, read_u16_be, u16}
    reader_impl! {read_u32_le, read_u32_be, u32}
    reader_impl! {read_u64_le, read_u64_be, u64}
    reader_impl! {read_u128_le, read_u128_be, u128}
}
