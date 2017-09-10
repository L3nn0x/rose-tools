//! Virtual File System
//!
//! ROSE Online uses a virtual file system to pack assets and ship with the
//! client. Assets are stored in binary blobs (.vfs) with an index (.idx)
//! containing the metadata for each asset file in the virtual files system.
//! Each file in the virtual file system maintains a 'filepath' which preserves
//! the directory hierarchy when unpacking the VFS.
//!
//! To interact with the virtual file system, start with the `VfsIndex` as it
//! contains all the information. It can be constructed manually in tandem
//! with a `.vfs` file or it can be loaded from disk.
//!
//! # Examples
//!
//! Load and interact with an existing index from a `.idx` file:
//!
//! ```rust,no_run
//! use std::path::Path;
//! use roseon::vfs::VfsIndex;
//!
//! let idx = VfsIndex::from_path(Path::new("/path/to/index.idx")).unwrap();
//!
//! for vfs in idx.file_systems {
//!     for vfs_file in vfs.files {
//!         println!("File: {}", vfs_file.filepath.to_str().unwrap_or(""));
//!     }
//! }
//! ```
use std::fs::File;
use std::io::{BufReader, BufWriter, Seek, SeekFrom};
use std::path::{PathBuf, Path};

use errors::*;
use io::{ReadRoseExt, WriteRoseExt, PathRoseExt};

/// Virtual file system index
///
/// An index of the virtual file systems, usually suffixed with `.idx`.
/// The index does not contain any actual asset data, only meta data about
/// the file systems. Each file system in the index usually maps to a single
/// `.vfs` file on disk.
#[derive(Debug, Serialize, Deserialize)]
pub struct VfsIndex {
    pub base_version: i32,
    pub current_version: i32,
    pub file_systems: Vec<VfsMetadata>,
}

/// Virtual file system
///
/// Contains the metadata for a single file system.
#[derive(Debug, Serialize, Deserialize)]
pub struct VfsMetadata {
    pub filename: PathBuf,
    pub files: Vec<VfsFileMetadata>,
}


/// Virtual file system file entry
///
/// Contains the metadata for a single file in the file system
#[derive(Debug, Serialize, Deserialize)]
pub struct VfsFileMetadata {
    pub filepath: PathBuf,
    pub offset: i32,
    pub size: i32,
    pub block_size: i32,
    pub is_deleted: bool,
    pub is_compressed: bool,
    pub is_encrypted: bool,
    pub version: i32,
    pub checksum: i32,
}

impl VfsIndex {
    /// Construct an empty `VfsIndex`
    ///
    /// # Usage
    /// ```rust
    /// use roseon::vfs::VfsIndex;
    ///
    /// let _ = VfsIndex::new();
    /// ```
    pub fn new() -> VfsIndex {
        VfsIndex {
            base_version: 0,
            current_version: 0,
            file_systems: Vec::new(),
        }
    }

    /// Construct a `VfsIndex` from a file
    ///
    /// # Usage
    /// ```rust,no_run
    /// use std::fs::File;
    /// use roseon::vfs::VfsIndex;
    ///
    /// let f = File::open("foo.idx").unwrap();
    /// let _ = VfsIndex::from_file(f);
    /// ```
    pub fn from_file(file: File) -> Result<VfsIndex> {
        let mut idx = VfsIndex::new();
        idx.load(file)?;
        Ok(idx)
    }

    /// Construct a `VfsIndex` from a path slice
    ///
    /// # Usage
    /// ```rust,no_run
    /// use std::path::PathBuf;
    /// use roseon::vfs::VfsIndex;
    ///
    /// let p = PathBuf::from("/path/to/my.idx");
    /// let _ = VfsIndex::from_path(&p);
    /// ```
    pub fn from_path(path: &Path) -> Result<VfsIndex> {
        let mut idx = VfsIndex::new();
        let f = File::open(path)?;
        idx.load(f)?;
        Ok(idx)
    }

    /// Load a `VfsIndex` from a file
    ///
    /// # Example
    /// ```rust,no_run
    /// use std::fs::File;
    /// use roseon::vfs::VfsIndex;
    ///
    /// let f = File::open("example.idx").unwrap();
    /// let mut idx = VfsIndex::new();
    /// idx.load(f).unwrap();
    ///
    /// for vfs in idx.file_systems {
    ///     for _ in vfs.files {
    ///         // Do something, e.g. copy bytes to local file system
    ///
    ///     }
    /// }
    /// ```
    pub fn load(&mut self, file: File) -> Result<()> {
        let mut reader = BufReader::new(file);
        self.load_reader(&mut reader)?;
        Ok(())
    }

    /// Save a `VfsIndex` to a file
    ///
    /// # Example
    /// ```rust,no_run
    /// use std::fs::File;
    /// use roseon::vfs::VfsIndex;
    ///
    /// let in_file = File::open("in.idx").unwrap();
    /// let out_file = File::open("out.idx").unwrap();
    ///
    /// let mut idx = VfsIndex::from_file(in_file).unwrap();
    ///
    /// // Do something with index
    /// idx.save(out_file).unwrap();
    /// ```
    pub fn save(&mut self, file: File) -> Result<()> {
        let mut writer = BufWriter::new(file);
        self.save_writer(&mut writer)?;
        Ok(())
    }

    /// Load a `VfsIndex` from a reader
    fn load_reader<R: ReadRoseExt + Seek>(&mut self, reader: &mut R) -> Result<()> {
        self.base_version = reader.read_i32()?;
        self.current_version = reader.read_i32()?;

        let vfs_count = reader.read_i32()?;
        for i in 0..vfs_count {
            let mut vfs = VfsMetadata::new();
            vfs.filename = PathBuf::from(reader.read_string_u16()?);

            let offset = reader.read_i32()?;
            let next_filesystem = reader.seek(SeekFrom::Current(0))?; // seek(0) returns current position
            let _ = reader.seek(SeekFrom::Start(offset as u64))?;

            let file_count = reader.read_i32()?;
            let _delete_count = reader.read_i32()?;
            let _start_offset = reader.read_i32()?;

            for _ in 0..file_count {
                let mut vfs_file = VfsFileMetadata::new();
                vfs_file.filepath = PathBuf::from_rose_path(&reader.read_string_u16()?);
                vfs_file.offset = reader.read_i32()?;
                vfs_file.size = reader.read_i32()?;
                vfs_file.block_size = reader.read_i32()?;
                vfs_file.is_deleted = reader.read_bool()?;
                vfs_file.is_compressed = reader.read_bool()?;
                vfs_file.is_encrypted = reader.read_bool()?;
                vfs_file.version = reader.read_i32()?;
                vfs_file.checksum = reader.read_i32()?;

                vfs.files.push(vfs_file);
            }

            self.file_systems.push(vfs);
            if i < vfs_count - 1 {
                let _ = reader.seek(SeekFrom::Start(next_filesystem as u64));
            }
        }
        Ok(())
    }

    /// Save a `VfsIndex` to a writer
    pub fn save_writer<W: WriteRoseExt + Seek>(&mut self, writer: &mut W) -> Result<()> {
        writer.write_i32(self.base_version)?;
        writer.write_i32(self.current_version)?;
        writer.write_i32(self.file_systems.len() as i32)?;

        let mut file_system_offsets: Vec<u64> = vec![];

        for i in 0..self.file_systems.len() {
            let fname = &self.file_systems[i].filename.to_str().unwrap_or("");
            writer.write_string_u16(fname)?;

            file_system_offsets.push(writer.seek(SeekFrom::Current(0))?);
            writer.write_i32(0)?; // Reserve to be written later
        }

        for i in 0..self.file_systems.len() {
            let ref vfs = self.file_systems[i];

            let file_offset = writer.seek(SeekFrom::Current(0))?;

            // Add data offset to header section
            writer.seek(SeekFrom::Start(file_system_offsets[i]))?;
            writer.write_i32(file_offset as i32)?;
            writer.seek(SeekFrom::Start(file_offset))?;

            let mut deleted_count: i32 = 0;
            for file in &vfs.files {
                if file.is_deleted {
                    deleted_count = deleted_count + 1;
                }
            }

            writer.write_i32(vfs.files.len() as i32)?;
            writer.write_i32(deleted_count)?;
            writer.write_i32(vfs.files[0].offset)?;

            for file in &vfs.files {
                let fname = &file.filepath.to_str().unwrap_or("");
                writer.write_string_u16(fname)?;
                writer.write_i32(file.offset)?;
                writer.write_i32(file.size)?;
                writer.write_i32(file.block_size)?;
                writer.write_bool(file.is_deleted)?;
                writer.write_bool(file.is_compressed)?;
                writer.write_bool(file.is_encrypted)?;
                writer.write_i32(file.version)?;
                writer.write_i32(file.checksum)?;
            }
        }
        Ok(())
    }
}

impl VfsMetadata {
    /// Construct an empty virtual file system
    pub fn new() -> VfsMetadata {
        VfsMetadata {
            filename: PathBuf::new(),
            files: Vec::new(),
        }
    }
}

impl VfsFileMetadata {
    /// Construct an empty virtual file system file
    pub fn new() -> VfsFileMetadata {
        VfsFileMetadata {
            filepath: PathBuf::new(),
            offset: 0,
            size: 0,
            block_size: 0,
            is_deleted: false,
            is_compressed: false,
            is_encrypted: false,
            version: 0,
            checksum: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::path::PathBuf;

    #[test]
    fn vfs_index_load() {
        let mut idx_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        idx_path.push("tests");
        idx_path.push("data");
        idx_path.push("data.idx");

        let idx = VfsIndex::from_path(&idx_path).unwrap();

        assert_eq!(idx.base_version, 129);
        assert_eq!(idx.current_version, 129);
        assert_eq!(idx.file_systems.len(), 2);

        let ref data_vfs = idx.file_systems[0];
        let ref data_vfs_last = data_vfs.files[data_vfs.files.len() - 1];

        assert_eq!(data_vfs.filename.to_str().unwrap(), "DATA.VFS");
        assert_eq!(data_vfs.files.len(), 3193);
        assert_eq!(data_vfs_last.filepath.to_str().unwrap(),
                   "3DDATA/EFFECT/_YETITYRANT_SKILL_01.EFT");

        let ref map_vfs = idx.file_systems[1];
        let ref map_vfs_last = map_vfs.files[map_vfs.files.len() - 1];

        assert_eq!(map_vfs.filename.to_str().unwrap(), "MAP.VFS");
        assert_eq!(map_vfs.files.len(), 11053);
        assert_eq!(map_vfs_last.filepath.to_str().unwrap(),
                   "3DDATA/TERRAIN/TILES/ZONETYPEINFO.STB");
    }

    #[test]
    fn vfs_index_save() {
        let mut idx_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        idx_path.push("tests");
        idx_path.push("data");
        idx_path.push("data.idx");

        let f = File::open(&idx_path).unwrap();
        let idx_size = f.metadata().unwrap().len();

        let mut orig_idx = VfsIndex::from_file(f).unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(idx_size as usize, 0u8);

        let mut cursor = Cursor::new(buffer);
        orig_idx.save_writer(&mut cursor).unwrap();

        // Load again to check save was successful
        cursor.set_position(0);
        let mut new_idx = VfsIndex::new();
        new_idx.load_reader(&mut cursor).unwrap();

        assert_eq!(new_idx.base_version, 129);
        assert_eq!(new_idx.current_version, 129);
        assert_eq!(new_idx.file_systems.len(), 2);

        let ref data_vfs = new_idx.file_systems[0];
        let ref data_vfs_last = data_vfs.files[data_vfs.files.len() - 1];

        assert_eq!(data_vfs.filename.to_str().unwrap(), "DATA.VFS");
        assert_eq!(data_vfs.files.len(), 3193);
        assert_eq!(data_vfs_last.filepath.to_str().unwrap(),
                   "3DDATA/EFFECT/_YETITYRANT_SKILL_01.EFT");

        let ref map_vfs = new_idx.file_systems[1];
        let ref map_vfs_last = map_vfs.files[map_vfs.files.len() - 1];

        assert_eq!(map_vfs.filename.to_str().unwrap(), "MAP.VFS");
        assert_eq!(map_vfs.files.len(), 11053);
        assert_eq!(map_vfs_last.filepath.to_str().unwrap(),
                   "3DDATA/TERRAIN/TILES/ZONETYPEINFO.STB");
    }
}
