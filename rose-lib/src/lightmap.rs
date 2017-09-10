//! Lightmap
//!
//! ROSE Online lightmap data used primarily for pre-baked lighting of maps
//!
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use errors::*;
use io::{ReadRoseExt, WriteRoseExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct Lightmap {
    pub objects: Vec<LightmapObject>,
    pub filenames: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapObject {
    pub id: i32,
    pub parts: Vec<LightmapPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapPart {
    pub name: String,
    pub id: i32,
    pub filename: String,
    pub lightmap_index: i32,
    pub pixels_per_part: i32,
    pub parts_per_width: i32,
    pub part_position: i32,
}

impl Lightmap {
    /// Construct an empty `Lightmap`
    ///
    /// # Usage
    /// ```rust
    /// use roseon::lightmap::Lightmap;
    ///
    /// let _ = Lightmap::new();
    /// ```
    pub fn new() -> Lightmap {
        Lightmap {
            objects: Vec::new(),
            filenames: Vec::new(),
        }
    }

    /// Construct a `Lightmap` from a file
    /// # Usage
    /// ```rust, no_run
    /// use roseon::lightmap::Lightmap;
    /// use std::fs::File;
    ///
    /// let f = File::open("lightmap.lit").unwrap();
    /// let _ = Lightmap::from_file(f);
    /// ```
    pub fn from_file(file: File) -> Result<Lightmap> {
        let mut lit = Lightmap::new();
        lit.load(file)?;
        Ok(lit)
    }

    /// Construct a `Lightmap` from a path slice
    ///
    /// # Usage
    /// ```rust,no_run
    /// use roseon::lightmap::Lightmap;
    /// use std::path::PathBuf;
    ///
    /// let p = PathBuf::from("path/to/my.lit");
    /// let _ = Lightmap::from_path(&p);
    /// ```
    pub fn from_path(path: &Path) -> Result<Lightmap> {
        let mut lit = Lightmap::new();
        let f = File::open(path)?;
        lit.load(f)?;
        Ok(lit)
    }

    /// Load a `Lightmap` from a file
    ///
    /// # Example
    /// ```rust,no_run
    /// use std::fs::File;
    /// use roseon::lightmap::Lightmap;
    ///
    /// let f = File::open("example.lit").unwrap();
    /// let mut lit = Lightmap::new();
    /// lit.load(f).unwrap();
    ///
    /// for obj in lit.objects {
    ///     for _ in obj.parts {
    ///         // Do something
    ///
    ///     }
    /// }
    /// ```
    pub fn load(&mut self, file: File) -> Result<()> {
        let mut reader = BufReader::new(file);
        self.load_reader(&mut reader)?;
        Ok(())
    }

    /// Save a `Lightmap` to a file
    ///
    /// # Example
    /// ```rust,no_run
    /// use std::fs::File;
    /// use roseon::lightmap::Lightmap;
    ///
    /// let in_file = File::open("in.lit").unwrap();
    /// let out_file = File::open("out.lit").unwrap();
    ///
    /// let mut lit = Lightmap::from_file(in_file).unwrap();
    ///
    /// // Do something with file
    /// lit.save(out_file).unwrap();
    /// ```
    pub fn save(&mut self, file: File) -> Result<()> {
        let mut writer = BufWriter::new(file);
        self.save_writer(&mut writer)?;
        Ok(())
    }

    /// Load a Lightmap from a reader
    fn load_reader<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<()> {
        let object_count = reader.read_i32()?;

        for _ in 0..object_count {
            let mut object = LightmapObject::new();

            let part_count = reader.read_i32()?;
            object.id = reader.read_i32()?;

            for _ in 0..part_count {
                let mut part = LightmapPart::new();
                part.name = reader.read_string_u8()?;
                part.id = reader.read_i32()?;
                part.filename = reader.read_string_u8()?;
                part.lightmap_index = reader.read_i32()?;
                part.pixels_per_part = reader.read_i32()?;
                part.parts_per_width = reader.read_i32()?;
                part.part_position = reader.read_i32()?;

                object.parts.push(part);
            }

            self.objects.push(object);
        }

        let file_count = reader.read_i32()?;

        for _ in 0..file_count {
            self.filenames.push(reader.read_string_u8()?);
        }

        Ok(())
    }

    /// Save a Lightmap to a writer
    fn save_writer<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<()> {
        writer.write_i32(self.objects.len() as i32)?;

        for ref object in &self.objects {
            writer.write_i32(object.parts.len() as i32)?;
            writer.write_i32(object.id)?;

            for ref part in &object.parts {
                writer.write_string_u8(&part.name)?;
                writer.write_i32(part.id)?;
                writer.write_string_u8(&part.filename)?;
                writer.write_i32(part.lightmap_index)?;
                writer.write_i32(part.pixels_per_part)?;
                writer.write_i32(part.parts_per_width)?;
                writer.write_i32(part.part_position)?;
            }
        }

        writer.write_i32(self.filenames.len() as i32)?;

        for ref filename in &self.filenames {
            writer.write_string_u8(&filename)?;
        }

        Ok(())
    }
}

impl LightmapObject {
    /// Construct an empty `LightmapObject`
    ///
    /// # Usage
    /// ```rust
    /// use roseon::lightmap::LightmapObject;
    ///
    /// let _ = LightmapObject::new();
    /// ```
    pub fn new() -> LightmapObject {
        LightmapObject {
            id: -1,
            parts: Vec::new(),
        }
    }
}

impl LightmapPart {
    /// Construct an empty `LightmapPart`
    ///
    /// # Usage
    /// ```rust
    /// use roseon::lightmap::LightmapPart;
    ///
    /// let _ = LightmapPart::new();
    /// ```
    pub fn new() -> LightmapPart {
        LightmapPart {
            name: String::new(),
            id: -1,
            filename: String::new(),
            lightmap_index: -1,
            pixels_per_part: 0,
            parts_per_width: 0,
            part_position: -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::path::PathBuf;

    #[test]
    fn lightmap_load() {
        let mut lit_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        lit_path.push("tests");
        lit_path.push("data");
        lit_path.push("OBJECTLIGHTMAPDATA.LIT");

        let lit = Lightmap::from_path(&lit_path).unwrap();

        assert_eq!(lit.objects.len(), 266);
        assert_eq!(lit.filenames.len(), 38);

        let ref first_obj = lit.objects[0];
        let ref first_part = first_obj.parts[0];
        let ref last_obj = lit.objects[lit.objects.len() - 1];
        let ref last_part = last_obj.parts[last_obj.parts.len() - 1];

        assert_eq!(first_obj.id, 1);
        assert_eq!(first_obj.parts.len(), 8);

        assert_eq!(first_part.name, "fountain_Object_1_0_32_32_LightingMap.tga");
        assert_eq!(first_part.id, 0);
        assert_eq!(first_part.filename, "Object_256_1.dds");
        assert_eq!(first_part.lightmap_index, 10);
        assert_eq!(first_part.pixels_per_part, 256);
        assert_eq!(first_part.parts_per_width, 2);
        assert_eq!(first_part.part_position, 2);

        assert_eq!(last_obj.id, 266);
        assert_eq!(last_obj.parts.len(), 1);

        assert_eq!(last_part.name,
                   "stonewall03_Object_266_0_32_32_LightingMap.tga");
        assert_eq!(last_part.id, 0);
        assert_eq!(last_part.filename, "Object_32_0.dds");
        assert_eq!(last_part.lightmap_index, 0);
        assert_eq!(last_part.pixels_per_part, 32);
        assert_eq!(last_part.parts_per_width, 16);
        assert_eq!(last_part.part_position, 52);
    }

    #[test]
    fn lightmap_save() {
        let mut lit_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        lit_path.push("tests");
        lit_path.push("data");
        lit_path.push("OBJECTLIGHTMAPDATA.LIT");

        let f = File::open(&lit_path).unwrap();
        let lit_size = f.metadata().unwrap().len();

        let mut orig_lit = Lightmap::from_path(&lit_path).unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(lit_size as usize, 0u8);

        let mut cursor = Cursor::new(buffer);
        orig_lit.save_writer(&mut cursor).unwrap();

        cursor.set_position(0);
        let mut new_lit = Lightmap::new();
        new_lit.load_reader(&mut cursor).unwrap();

        assert_eq!(new_lit.objects.len(), 266);
        assert_eq!(new_lit.filenames.len(), 38);

        let ref first_obj = new_lit.objects[0];
        let ref first_part = first_obj.parts[0];
        let ref last_obj = new_lit.objects[new_lit.objects.len() - 1];
        let ref last_part = last_obj.parts[last_obj.parts.len() - 1];

        assert_eq!(first_obj.id, 1);
        assert_eq!(first_obj.parts.len(), 8);

        assert_eq!(first_part.name, "fountain_Object_1_0_32_32_LightingMap.tga");
        assert_eq!(first_part.id, 0);
        assert_eq!(first_part.filename, "Object_256_1.dds");
        assert_eq!(first_part.lightmap_index, 10);
        assert_eq!(first_part.pixels_per_part, 256);
        assert_eq!(first_part.parts_per_width, 2);
        assert_eq!(first_part.part_position, 2);

        assert_eq!(last_obj.id, 266);
        assert_eq!(last_obj.parts.len(), 1);

        assert_eq!(last_part.name,
                   "stonewall03_Object_266_0_32_32_LightingMap.tga");
        assert_eq!(last_part.id, 0);
        assert_eq!(last_part.filename, "Object_32_0.dds");
        assert_eq!(last_part.lightmap_index, 0);
        assert_eq!(last_part.pixels_per_part, 32);
        assert_eq!(last_part.parts_per_width, 16);
        assert_eq!(last_part.part_position, 52);
    }
}
