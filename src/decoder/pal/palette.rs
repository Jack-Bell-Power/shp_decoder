use std::path::Path;

use super::pal_color::PalColor;

pub struct Palette {
    /// The 256 colors in palette
    pub colors: [PalColor; 256],
}

impl Palette {
    pub fn get_color(&self, index: u8) -> anyhow::Result<PalColor> {
        match self.colors.get(index as usize) {
            Some(s) => Ok(*s),
            None => Err(anyhow::anyhow!("Out of range.")),
        }
    }

    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let bytes = std::fs::read(path)?;
        Self::decode(&bytes)
    }

    pub fn decode(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() != 256 * 3 {
            return Err(anyhow::anyhow!(
                "The byte array length is incorrect; the PAL file should be 256 * 3 bytes."
            ));
        }

        let mut colors: [PalColor; 256] = [PalColor {
            red: 0,
            green: 0,
            blue: 0,
        }; 256];

        for i in 0..256 {
            colors[i].red = bytes[i * 3];
            colors[i].green = bytes[i * 3 + 1];
            colors[i].blue = bytes[i * 3 + 2];
        }

        Ok(Palette { colors })
    }
}
