use std::path::Path;

use crate::errors::Ra2Error;

use super::pal_color::PalColor;

pub struct Palette {
    /// The 256 colors in palette
    pub colors: [PalColor; 256],
}

impl Palette {
    pub fn get_color(&self, index: u8) -> Result<PalColor, Ra2Error> {
        match self.colors.get(index as usize) {
            Some(s) => Ok(*s),
            None => Err(Ra2Error::InvalidFormat {
                message: "Out of range.".to_string(),
            }),
        }
    }

    pub fn load(path: &Path) -> Result<Self, Ra2Error> {
        let bytes = std::fs::read(path)?;
        Self::decode(&bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, Ra2Error> {
        if bytes.len() != 256 * 3 {
            return Err(Ra2Error::InvalidFormat {
                message:
                    "The byte array length is incorrect; the PAL file should be 256 * 3 bytes."
                        .to_string(),
            });
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
