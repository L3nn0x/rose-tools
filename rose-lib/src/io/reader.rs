use std::io::{BufRead, Read, Seek};
use byteorder::{ReadBytesExt, LittleEndian};

use errors::*;
use utils::{Color4, Vector2, Vector3, Vector4};

/// Extends `BufReader` with methods for reading ROSE data types
///
///# Example
/// ```rust,no_run
/// use std::fs::File;
/// use std::io::BufReader;
/// use roselib::io::ReadRoseExt;
///
/// let f = File::open("my_file.ext").unwrap();
/// let mut reader = BufReader::new(f);
/// let x = reader.read_i8().unwrap();
/// let y = reader.read_f64().unwrap();
///
/// let s = reader.read_cstring().unwrap();
///
/// println!("x is {}, y is {}, s is {}", x, y, s);
/// ```
///
/// NOTE: Strings are encoded as UTF-8 and the original ROSE files were encoded
/// as EUC-KR, as such some string data may be lost.
pub trait ReadRoseExt: Read + Seek + BufRead {
    fn read_u8(&mut self) -> Result<u8>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u32(&mut self) -> Result<u32>;

    fn read_i8(&mut self) -> Result<i8>;
    fn read_i16(&mut self) -> Result<i16>;
    fn read_i32(&mut self) -> Result<i32>;

    fn read_bool(&mut self) -> Result<bool>;
    fn read_f32(&mut self) -> Result<f32>;
    fn read_f64(&mut self) -> Result<f64>;

    /// Read a null-terminated (c-style string) from the reader
    fn read_cstring(&mut self) -> Result<String>;

    /// Read a string of n-bytes length from the reader
    fn read_string(&mut self, n: u64) -> Result<String>;

    /// Read a string with a u8 prefixed length from the reader
    fn read_string_u8(&mut self) -> Result<String>;

    /// Read a string with a u16 prefixed length from the reader
    fn read_string_u16(&mut self) -> Result<String>;

    /// Read a string with a u32 prefixed length from the reader
    fn read_string_u32(&mut self) -> Result<String>;

    fn read_color4(&mut self) -> Result<Color4>;

    fn read_vector2_f32(&mut self) -> Result<Vector2<f32>>;
    fn read_vector3_f32(&mut self) -> Result<Vector3<f32>>;
    fn read_vector3_i16(&mut self) -> Result<Vector3<i16>>;
    fn read_vector4_f32(&mut self) -> Result<Vector4<f32>>;
    fn read_vector4_i16(&mut self) -> Result<Vector4<i16>>;
}

impl<R> ReadRoseExt for R
    where R: Read,
          R: Seek,
          R: BufRead,
          R: ReadBytesExt
{
    fn read_u8(&mut self) -> Result<u8> {
        Ok(ReadBytesExt::read_u8(self)?)
    }

    fn read_u16(&mut self) -> Result<u16> {
        Ok(ReadBytesExt::read_u16::<LittleEndian>(self)?)
    }

    fn read_u32(&mut self) -> Result<u32> {
        Ok(ReadBytesExt::read_u32::<LittleEndian>(self)?)
    }

    fn read_i8(&mut self) -> Result<i8> {
        Ok(ReadBytesExt::read_i8(self)?)
    }

    fn read_i16(&mut self) -> Result<i16> {
        Ok(ReadBytesExt::read_i16::<LittleEndian>(self)?)
    }

    fn read_i32(&mut self) -> Result<i32> {
        Ok(ReadBytesExt::read_i32::<LittleEndian>(self)?)
    }

    fn read_bool(&mut self) -> Result<bool> {
        let b = ReadRoseExt::read_u8(self)?;
        match b {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    fn read_f32(&mut self) -> Result<f32> {
        Ok(ReadBytesExt::read_f32::<LittleEndian>(self)?)
    }

    fn read_f64(&mut self) -> Result<f64> {
        Ok(ReadBytesExt::read_f64::<LittleEndian>(self)?)
    }

    fn read_cstring(&mut self) -> Result<String> {
        let mut buffer: Vec<u8> = Vec::new();
        self.read_until(0x00, &mut buffer)?;
        let _ = buffer.pop();
        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }

    fn read_string(&mut self, n: u64) -> Result<String> {
        let mut buffer = Vec::new();
        let mut bytes = self.take(n as u64);
        bytes.read_to_end(&mut buffer)?;

        // Remove terminating null bytes
        if let Some(&0x00) = buffer.last() {
            let _ = buffer.pop();
        }

        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }

    fn read_string_u8(&mut self) -> Result<String> {
        let length = ReadRoseExt::read_u8(self)?;
        self.read_string(length as u64)
    }

    fn read_string_u16(&mut self) -> Result<String> {
        let length = ReadRoseExt::read_u16(self)?;
        self.read_string(length as u64)
    }

    fn read_string_u32(&mut self) -> Result<String> {
        let length = ReadRoseExt::read_u32(self)?;
        self.read_string(length as u64)
    }

    fn read_color4(&mut self) -> Result<Color4> {
        let mut c = Color4::new();
        c.r = ReadRoseExt::read_f32(self)?;
        c.g = ReadRoseExt::read_f32(self)?;
        c.b = ReadRoseExt::read_f32(self)?;
        c.a = ReadRoseExt::read_f32(self)?;
        Ok(c)
    }

    fn read_vector2_f32(&mut self) -> Result<Vector2<f32>> {
        let mut v = Vector2::<f32>::new();
        v.x = ReadRoseExt::read_f32(self)?;
        v.y = ReadRoseExt::read_f32(self)?;
        Ok(v)
    }

    fn read_vector3_f32(&mut self) -> Result<Vector3<f32>> {
        let mut v = Vector3::<f32>::new();
        v.x = ReadRoseExt::read_f32(self)?;
        v.y = ReadRoseExt::read_f32(self)?;
        v.z = ReadRoseExt::read_f32(self)?;
        Ok(v)
    }

    fn read_vector3_i16(&mut self) -> Result<Vector3<i16>> {
        let mut v = Vector3::<i16>::new();
        v.x = ReadRoseExt::read_i16(self)?;
        v.y = ReadRoseExt::read_i16(self)?;
        v.z = ReadRoseExt::read_i16(self)?;
        Ok(v)
    }
    fn read_vector4_f32(&mut self) -> Result<Vector4<f32>> {
        let mut v = Vector4::<f32>::new();
        v.w = ReadRoseExt::read_f32(self)?;
        v.x = ReadRoseExt::read_f32(self)?;
        v.y = ReadRoseExt::read_f32(self)?;
        v.z = ReadRoseExt::read_f32(self)?;
        Ok(v)
    }

    fn read_vector4_i16(&mut self) -> Result<Vector4<i16>> {
        let mut v = Vector4::<i16>::new();
        v.w = ReadRoseExt::read_i16(self)?;
        v.x = ReadRoseExt::read_i16(self)?;
        v.y = ReadRoseExt::read_i16(self)?;
        v.z = ReadRoseExt::read_i16(self)?;
        Ok(v)
    }
}
