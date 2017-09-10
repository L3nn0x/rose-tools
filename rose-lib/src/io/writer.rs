use std::io::Write;
use byteorder::{WriteBytesExt, LittleEndian};

use errors::*;
use utils::{Color4, Vector2, Vector3, Vector4};

/// Extends `BufWriter` with methods for writing ROSE data types
///
///# Example
/// ```rust,no_run
/// use std::fs::File;
/// use std::io::BufWriter;
/// use roseon::io::WriteRoseExt;
///
/// let f = File::open("my_file.ext").unwrap();
/// let mut writer = BufWriter::new(f);
/// writer.write_i8(5i8).unwrap();
/// writer.write_f64(3.14f64).unwrap();
///
/// writer.write_cstring("null terminate me").unwrap();
/// ```
///
/// NOTE: Strings are encoded as UTF-8 and no UTF-8 strings are lossily encoded
/// into UTF-8. The original ROSE files were encoded as EUC-KR, as such some
/// data may be lost.
pub trait WriteRoseExt {
    fn write_u8(&mut self, n: u8) -> Result<()>;
    fn write_u16(&mut self, n: u16) -> Result<()>;
    fn write_u32(&mut self, n: u32) -> Result<()>;

    fn write_i8(&mut self, n: i8) -> Result<()>;
    fn write_i16(&mut self, n: i16) -> Result<()>;
    fn write_i32(&mut self, n: i32) -> Result<()>;

    fn write_bool(&mut self, b: bool) -> Result<()>;
    fn write_f32(&mut self, n: f32) -> Result<()>;
    fn write_f64(&mut self, n: f64) -> Result<()>;

    // Write string as null terminated string
    fn write_cstring(&mut self, string: &str) -> Result<()>;

    // Write a string with length prefix as u8
    fn write_string_u8(&mut self, string: &str) -> Result<()>;

    // Write a string with length prefix as u16
    fn write_string_u16(&mut self, string: &str) -> Result<()>;

    // Write a string with length prefix as u32
    fn write_string_u32(&mut self, string: &str) -> Result<()>;

    fn write_color4(&mut self, color: &Color4) -> Result<()>;

    fn write_vector2_f32(&mut self, v: &Vector2<f32>) -> Result<()>;
    fn write_vector3_f32(&mut self, v: &Vector3<f32>) -> Result<()>;
    fn write_vector3_i16(&mut self, v: &Vector3<i16>) -> Result<()>;
    fn write_vector4_f32(&mut self, v: &Vector4<f32>) -> Result<()>;
    fn write_vector4_i16(&mut self, v: &Vector4<i16>) -> Result<()>;
}

impl<W> WriteRoseExt for W
    where W: Write,
          W: WriteBytesExt
{
    fn write_u8(&mut self, n: u8) -> Result<()> {
        WriteBytesExt::write_u8(self, n)?;
        Ok(())
    }

    fn write_u16(&mut self, n: u16) -> Result<()> {
        WriteBytesExt::write_u16::<LittleEndian>(self, n)?;
        Ok(())
    }

    fn write_u32(&mut self, n: u32) -> Result<()> {
        WriteBytesExt::write_u32::<LittleEndian>(self, n)?;
        Ok(())
    }

    fn write_i8(&mut self, n: i8) -> Result<()> {
        WriteBytesExt::write_i8(self, n)?;
        Ok(())
    }

    fn write_i16(&mut self, n: i16) -> Result<()> {
        WriteBytesExt::write_i16::<LittleEndian>(self, n)?;
        Ok(())
    }

    fn write_i32(&mut self, n: i32) -> Result<()> {
        WriteBytesExt::write_i32::<LittleEndian>(self, n)?;
        Ok(())
    }

    fn write_bool(&mut self, b: bool) -> Result<()> {
        let i = if b == true { 1u8 } else { 0u8 };
        WriteRoseExt::write_u8(self, i)?;
        Ok(())
    }

    fn write_f32(&mut self, n: f32) -> Result<()> {
        WriteBytesExt::write_f32::<LittleEndian>(self, n)?;
        Ok(())
    }

    fn write_f64(&mut self, n: f64) -> Result<()> {
        WriteBytesExt::write_f64::<LittleEndian>(self, n)?;
        Ok(())
    }

    fn write_cstring(&mut self, string: &str) -> Result<()> {
        self.write_all(string.as_bytes())?;
        WriteRoseExt::write_u8(self, 0x00)?;
        Ok(())
    }

    fn write_string_u8(&mut self, string: &str) -> Result<()> {
        WriteRoseExt::write_u8(self, string.len() as u8)?;
        self.write_all(string.as_bytes())?;
        Ok(())
    }

    fn write_string_u16(&mut self, string: &str) -> Result<()> {
        WriteRoseExt::write_u16(self, string.len() as u16)?;
        self.write_all(string.as_bytes())?;
        Ok(())
    }

    fn write_string_u32(&mut self, string: &str) -> Result<()> {
        WriteRoseExt::write_u32(self, string.len() as u32)?;
        self.write_all(&string.as_bytes())?;
        Ok(())
    }

    fn write_color4(&mut self, color: &Color4) -> Result<()> {
        WriteRoseExt::write_f32(self, color.r)?;
        WriteRoseExt::write_f32(self, color.g)?;
        WriteRoseExt::write_f32(self, color.b)?;
        WriteRoseExt::write_f32(self, color.a)?;
        Ok(())
    }

    fn write_vector2_f32(&mut self, v: &Vector2<f32>) -> Result<()> {
        WriteRoseExt::write_f32(self, v.x)?;
        WriteRoseExt::write_f32(self, v.y)?;
        Ok(())
    }

    fn write_vector3_f32(&mut self, v: &Vector3<f32>) -> Result<()> {
        WriteRoseExt::write_f32(self, v.x)?;
        WriteRoseExt::write_f32(self, v.y)?;
        WriteRoseExt::write_f32(self, v.z)?;
        Ok(())
    }

    fn write_vector3_i16(&mut self, v: &Vector3<i16>) -> Result<()> {
        WriteRoseExt::write_i16(self, v.x)?;
        WriteRoseExt::write_i16(self, v.y)?;
        WriteRoseExt::write_i16(self, v.z)?;
        Ok(())
    }

    fn write_vector4_f32(&mut self, v: &Vector4<f32>) -> Result<()> {
        WriteRoseExt::write_f32(self, v.w)?;
        WriteRoseExt::write_f32(self, v.x)?;
        WriteRoseExt::write_f32(self, v.y)?;
        WriteRoseExt::write_f32(self, v.z)?;
        Ok(())
    }

    fn write_vector4_i16(&mut self, v: &Vector4<i16>) -> Result<()> {
        WriteRoseExt::write_i16(self, v.w)?;
        WriteRoseExt::write_i16(self, v.x)?;
        WriteRoseExt::write_i16(self, v.y)?;
        WriteRoseExt::write_i16(self, v.z)?;
        Ok(())
    }
}
