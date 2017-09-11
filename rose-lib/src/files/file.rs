use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use errors::*;
use io::{ReadRoseExt, WriteRoseExt};

pub trait RoseFile {
    // -- Constructors
    fn new() -> Self;


    fn from_file(file: File) -> Result<Self>
        where Self: Sized
    {
        let mut rf = Self::new();
        rf.load(file)?;
        Ok(rf)
    }

    fn from_path(path: &Path) -> Result<Self>
        where Self: Sized
    {
        let mut rf = Self::new();
        let f = File::open(path)?;
        rf.load(f)?;
        Ok(rf)
    }

    fn from_reader<R: ReadRoseExt>(reader: &mut R) -> Result<Self>
        where Self: Sized
    {
        let mut rf = Self::new();
        rf.read(reader)?;
        Ok(rf)
    }

    // -- Methods
    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<()>;
    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<()>;

    fn load(&mut self, file: File) -> Result<()> {
        let mut reader = BufReader::new(file);
        self.read(&mut reader)?;
        Ok(())
    }

    fn save(&mut self, file: File) -> Result<()> {
        let mut writer = BufWriter::new(file);
        self.write(&mut writer)?;
        Ok(())
    }
}
