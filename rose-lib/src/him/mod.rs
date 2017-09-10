use std::f32;
use errors::*;
use io::ReadRoseExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Heightmap {
    pub width: i32,
    pub height: i32,
    pub grid_count: i32,
    pub scale: f32,

    pub heights: Vec<Vec<f32>>,

    pub min_height: f32,
    pub max_height: f32,
}

impl Heightmap {
    pub fn new() -> Heightmap {
        Heightmap {
            width: 0,
            height: 0,
            grid_count: 0,
            scale: 0.0,
            heights: Vec::new(),

            min_height: f32::NAN,
            max_height: f32::NAN,
        }
    }

    pub fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<()> {
        self.width = reader.read_i32()?;
        self.height = reader.read_i32()?;
        self.grid_count = reader.read_i32()?;
        self.scale = reader.read_f32()?;

        self.heights = vec![vec![0.0; self.width as usize]; self.height as usize];
        for h in 0..self.height {
            for w in 0..self.width {
                let h = h as usize;
                let w = w as usize;

                let height = reader.read_f32()?;

                self.heights[h][w] = height;

                if self.min_height.is_nan() || height < self.min_height {
                    self.min_height = height;
                }

                if self.max_height.is_nan() || height > self.max_height {
                    self.max_height = height;
                }
            }
        }

        // TODO: File contains more data but we probably don't care

        Ok(())

    }
}
