//! Model
//!
//! ROSE Online 3D model
use files::RoseFile;
use utils::{BoundingBox, Color4, Vector2, Vector3, Vector4};
use io::{ReadRoseExt, WriteRoseExt};
use errors::*;

pub type ZMS = ModelFile;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ModelFile {
    pub identifier: String,
    pub format: i32,

    pub bounding_box: BoundingBox<f32>,
    pub bones: Vec<i16>,
    pub vertices: Vec<ModelVertex>,
    pub indices: Vec<Vector3<i16>>,
    pub materials: Vec<i16>,
    pub strips: Vec<i16>,

    // Pool properties for the vertex buffer [Static/Dynamic/System]
    pub pool: i16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ModelVertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub color: Color4,
    pub bone_weights: Vector4<f32>,
    pub bone_indices: Vector4<i16>,
    pub tangent: Vector3<f32>,
    pub uv1: Vector2<f32>,
    pub uv2: Vector2<f32>,
    pub uv3: Vector2<f32>,
    pub uv4: Vector2<f32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum VertFormat {
    Position = 1 << 1,
    Normal = 1 << 2,
    Color = 1 << 3,
    BoneWeight = 1 << 4,
    BoneIndex = 1 << 5,
    Tangent = 1 << 6,
    UV1 = 1 << 7,
    UV2 = 1 << 8,
    UV3 = 1 << 9,
    UV4 = 1 << 10,
}

impl ModelFile {
    /// Check if `ModelFile` has position vertices enabled
    pub fn positions_enabled(&self) -> bool {
        (VertFormat::Position as i32 & self.format) != 0
    }

    /// Check if `ModelFile` has normal vertices enabled
    pub fn normals_enabled(&self) -> bool {
        (VertFormat::Normal as i32 & self.format) != 0
    }

    /// Check if `ModelFile` has color vertices enabled
    pub fn colors_enabled(&self) -> bool {
        (VertFormat::Color as i32 & self.format) != 0
    }

    /// Check if `ModelFile` has bones enabled
    pub fn bones_enabled(&self) -> bool {
        ((VertFormat::BoneWeight as i32 & self.format) != 0) &&
        ((VertFormat::BoneIndex as i32 & self.format) != 0)
    }

    /// Check if `ModelFile` has color tangents enabled
    pub fn tangents_enabled(&self) -> bool {
        (VertFormat::Tangent as i32 & self.format) != 0
    }

    /// Check if `ModelFile` has texture uv1 enabled
    pub fn uv1_enabled(&self) -> bool {
        (VertFormat::UV1 as i32 & self.format) != 0
    }

    /// Check if `ModelFile` has texture uv2 enabled
    pub fn uv2_enabled(&self) -> bool {
        (VertFormat::UV2 as i32 & self.format) != 0
    }

    /// Check if `ModelFile` has texture uv3 enabled
    pub fn uv3_enabled(&self) -> bool {
        (VertFormat::UV3 as i32 & self.format) != 0
    }

    /// Check if `ModelFile` has texture uv4 enabled
    pub fn uv4_enabled(&self) -> bool {
        (VertFormat::UV4 as i32 & self.format) != 0
    }
}

impl ModelVertex {
    pub fn new() -> ModelVertex {
        ModelVertex {
            position: Vector3::<f32>::new(),
            normal: Vector3::<f32>::new(),
            color: Color4::new(),
            bone_weights: Vector4::<f32>::new(),
            bone_indices: Vector4::<i16>::new(),
            tangent: Vector3::<f32>::new(),
            uv1: Vector2::<f32>::new(),
            uv2: Vector2::<f32>::new(),
            uv3: Vector2::<f32>::new(),
            uv4: Vector2::<f32>::new(),
        }
    }
}


impl RoseFile for ModelFile {
    /// Construct an empty `ModelFile`
    ///
    /// # Usage
    /// ```rust
    /// use roselib::files::{RoseFile, ZMS};
    ///
    /// let _ = ZMS::new();
    /// ```
    fn new() -> ModelFile {
        ModelFile {
            identifier: String::from(""),
            format: -1,
            bounding_box: BoundingBox {
                min: Vector3::<f32>::new(),
                max: Vector3::<f32>::new(),
            },
            bones: Vec::new(),
            vertices: Vec::new(),
            indices: Vec::new(),
            materials: Vec::new(),
            strips: Vec::new(),
            pool: 0,
        }
    }

    /// Read data from a reader
    ///
    /// # Usage
    /// ```rust
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use roselib::files::{RoseFile, ZMS};
    ///
    /// # fn foo() {
    /// let f = File::open("foo.zms").unwrap();
    /// let mut r = BufReader::new(f);
    /// let mut z = ZMS::new();
    /// z.read(&mut r);
    /// # }
    /// ```
    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<()> {
        self.identifier = reader.read_cstring()?;

        let version = match self.identifier.as_str() {
            "ZMS0007" => 7,
            "ZMS0008" => 8,
            _ => return Err("Unsupported ZMS version".into()),
        };

        self.format = reader.read_i32()?;
        self.bounding_box.min = reader.read_vector3_f32()?;
        self.bounding_box.max = reader.read_vector3_f32()?;

        let bone_count = reader.read_i16()?;
        for _ in 0..bone_count {
            self.bones.push(reader.read_i16()?);
        }

        let vert_count = reader.read_i16()?;
        for _ in 0..vert_count {
            self.vertices.push(ModelVertex::new());
        }

        if self.positions_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].position = reader.read_vector3_f32()?;
            }
        }

        if self.normals_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].normal = reader.read_vector3_f32()?;
            }
        }

        if self.colors_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].color = reader.read_color4()?;
            }
        }

        if self.bones_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].bone_weights = reader.read_vector4_f32()?;
                self.vertices[i].bone_indices = reader.read_vector4_i16()?;

            }
        }

        if self.tangents_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].tangent = reader.read_vector3_f32()?;
            }
        }

        if self.uv1_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].uv1 = reader.read_vector2_f32()?;
            }
        }

        if self.uv2_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].uv2 = reader.read_vector2_f32()?;
            }
        }

        if self.uv3_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].uv3 = reader.read_vector2_f32()?;
            }
        }
        if self.uv4_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].uv4 = reader.read_vector2_f32()?;
            }
        }

        let index_count = reader.read_i16()?;
        for _ in 0..index_count {
            self.indices.push(reader.read_vector3_i16()?);
        }

        let material_count = reader.read_i16()?;
        for _ in 0..material_count {
            self.materials.push(reader.read_i16()?);
        }

        let strip_count = reader.read_i16()?;
        for _ in 0..strip_count {
            self.strips.push(reader.read_i16()?);
        }

        if version >= 8 {
            self.pool = reader.read_i16()?;
        }

        Ok(())
    }

    /// Save a Model to a writer
    ///
    /// # Usage
    /// ```rust
    /// use std::fs::File;
    /// use std::io::BufWriter;
    /// use roselib::files::{RoseFile,ZMS};
    ///
    /// # fn foo() {
    /// let f = File::open("foo.zms").unwrap();
    /// let mut w = BufWriter::new(f);
    /// let mut z = ZMS::new();
    /// z.write(&mut w);
    /// # }
    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<()> {
        writer.write_cstring("ZMS0008")?;
        writer.write_i32(self.format)?;

        writer.write_vector3_f32(&self.bounding_box.min)?;
        writer.write_vector3_f32(&self.bounding_box.max)?;

        writer.write_i16(self.bones.len() as i16)?;
        for bone in &self.bones {
            writer.write_i16(*bone)?;
        }

        writer.write_i16(self.vertices.len() as i16)?;

        if self.positions_enabled() {
            for ref vertex in &self.vertices {
                writer.write_vector3_f32(&vertex.position)?;
            }
        }

        if self.normals_enabled() {
            for ref vertex in &self.vertices {
                writer.write_vector3_f32(&vertex.normal)?;
            }
        }

        if self.colors_enabled() {
            for ref vertex in &self.vertices {
                writer.write_color4(&vertex.color)?;
            }
        }

        if self.bones_enabled() {
            for ref vertex in &self.vertices {
                writer.write_vector4_f32(&vertex.bone_weights)?;
                writer.write_vector4_i16(&vertex.bone_indices)?;
            }
        }

        if self.tangents_enabled() {
            for ref vertex in &self.vertices {
                writer.write_vector3_f32(&vertex.tangent)?;
            }
        }

        if self.uv1_enabled() {
            for ref vertex in &self.vertices {
                writer.write_vector2_f32(&vertex.uv1)?;
            }
        }

        if self.uv2_enabled() {
            for ref vertex in &self.vertices {
                writer.write_vector2_f32(&vertex.uv2)?;
            }
        }

        if self.uv3_enabled() {
            for ref vertex in &self.vertices {
                writer.write_vector2_f32(&vertex.uv3)?;
            }
        }

        if self.uv4_enabled() {
            for ref vertex in &self.vertices {
                writer.write_vector2_f32(&vertex.uv4)?;
            }
        }

        writer.write_i16(self.indices.len() as i16)?;
        for index in &self.indices {
            writer.write_vector3_i16(index)?;
        }

        writer.write_i16(self.materials.len() as i16)?;
        for mat in &self.materials {
            writer.write_i16(*mat)?;
        }

        writer.write_i16(self.strips.len() as i16)?;
        for strip in &self.strips {
            writer.write_i16(*strip)?;
        }

        writer.write_i16(self.pool)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Cursor;
    use std::path::PathBuf;

    #[test]
    fn model_load() {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.push("tests");
        root.push("data");

        let file1 = root.join("HEADBAD01.ZMS");
        let file2 = root.join("STONE014.ZMS");
        let file3 = root.join("CART01_ABILITY01.ZMS");

        let model1 = ModelFile::from_path(&file1).unwrap();
        assert_eq!(model1.identifier.as_str(), "ZMS0008");
        assert_eq!(model1.format, 182);
        assert_eq!(model1.positions_enabled(), true);
        assert_eq!(model1.normals_enabled(), true);
        assert_eq!(model1.colors_enabled(), false);
        assert_eq!(model1.bones_enabled(), true);
        assert_eq!(model1.tangents_enabled(), false);
        assert_eq!(model1.uv1_enabled(), true);
        assert_eq!(model1.uv2_enabled(), false);
        assert_eq!(model1.uv3_enabled(), false);
        assert_eq!(model1.uv4_enabled(), false);

        assert_eq!(model1.bones.len(), 8);
        assert_eq!(model1.vertices.len(), 336);
        assert_eq!(model1.indices.len(), 578);
        assert_eq!(model1.materials.len(), 6);
        assert_eq!(model1.strips.len(), 0);
        assert_eq!(model1.pool, 0);

        let model2 = ModelFile::from_path(&file2).unwrap();
        assert_eq!(model2.identifier.as_str(), "ZMS0007");
        assert_eq!(model2.format, 390);
        assert_eq!(model2.positions_enabled(), true);
        assert_eq!(model2.normals_enabled(), true);
        assert_eq!(model2.colors_enabled(), false);
        assert_eq!(model2.bones_enabled(), false);
        assert_eq!(model2.tangents_enabled(), false);
        assert_eq!(model2.uv1_enabled(), true);
        assert_eq!(model2.uv2_enabled(), true);
        assert_eq!(model2.uv3_enabled(), false);
        assert_eq!(model2.uv4_enabled(), false);

        assert_eq!(model2.bones.len(), 0);
        assert_eq!(model2.vertices.len(), 131);
        assert_eq!(model2.indices.len(), 128);
        assert_eq!(model2.materials.len(), 0);
        assert_eq!(model2.strips.len(), 0);
        assert_eq!(model2.pool, 0);

        let model3 = ModelFile::from_path(&file3).unwrap();
        assert_eq!(model3.identifier.as_str(), "ZMS0008");
        assert_eq!(model3.format, 134);
        assert_eq!(model3.positions_enabled(), true);
        assert_eq!(model3.normals_enabled(), true);
        assert_eq!(model3.colors_enabled(), false);
        assert_eq!(model3.bones_enabled(), false);
        assert_eq!(model3.tangents_enabled(), false);
        assert_eq!(model3.uv1_enabled(), true);
        assert_eq!(model3.uv2_enabled(), false);
        assert_eq!(model3.uv3_enabled(), false);
        assert_eq!(model3.uv4_enabled(), false);

        assert_eq!(model3.bones.len(), 0);
        assert_eq!(model3.vertices.len(), 544);
        assert_eq!(model3.indices.len(), 532);
        assert_eq!(model3.materials.len(), 2);
        assert_eq!(model3.strips.len(), 0);
        assert_eq!(model3.pool, 0);
    }

    #[test]
    fn model_save() {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.push("tests");
        root.push("data");

        let file1 = root.join("HEADBAD01.ZMS");
        let file2 = root.join("STONE014.ZMS");
        let file3 = root.join("CART01_ABILITY01.ZMS");

        for zms_file in [file1, file2, file3].iter() {
            let f = File::open(&zms_file).unwrap();
            let zms_size = f.metadata().unwrap().len();

            let mut orig_zms = ModelFile::from_path(&zms_file).unwrap();

            let mut buffer: Vec<u8> = Vec::new();
            buffer.resize(zms_size as usize, 0u8);

            let mut cursor = Cursor::new(buffer);
            orig_zms.write(&mut cursor).unwrap();

            cursor.set_position(0);

            let mut new_zms = ModelFile::new();
            new_zms.read(&mut cursor).unwrap();

            if orig_zms.identifier.as_str() == "ZMS0007" {
                orig_zms.identifier = String::from("ZMS0008");
            }

            assert_eq!(orig_zms, new_zms);
        }
    }
}
