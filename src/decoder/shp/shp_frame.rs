use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use image::{Rgba, RgbaImage};

use crate::decoder::pal::palette::Palette;

#[derive(Default)]
pub struct ShpFrame {
    // The starting coordinate of the x-axis
    pub x: u16,
    // The starting coordinate of the y-axis
    pub y: u16,
    // The width of this frame
    pub width: u16,
    // The height of this frame
    pub height: u16,
    //
    pub flags: u8,
    // unused
    pub reserved1: [u8; 3],
    // unused
    pub color: u32,
    // unused
    pub reserved2: [u8; 4],
    // Offset start
    pub offset: u32,
    // The index buffer
    pub buffer: Vec<u8>,
}

impl ShpFrame {
    pub fn read_shp_frame_header<R: Read>(&mut self, reader: &mut R) -> anyhow::Result<()> {
        self.x = reader.read_u16::<LittleEndian>()?;
        self.y = reader.read_u16::<LittleEndian>()?;
        self.width = reader.read_u16::<LittleEndian>()?;
        self.height = reader.read_u16::<LittleEndian>()?;
        self.flags = reader.read_u8()?;
        reader.read_exact(&mut self.reserved1)?;
        self.color = reader.read_u32::<LittleEndian>()?;
        reader.read_exact(&mut self.reserved2)?;
        self.offset = reader.read_u32::<LittleEndian>()?;
        Ok(())
    }

    pub fn read_shp_frame_data<R: Read + Seek>(&mut self, reader: &mut R) -> anyhow::Result<()> {
        //If the offset is 0, it indicates an empty frame.
        if self.offset == 0 {
            return Ok(());
        }
        //Jump to the offset position of the frame data.
        reader.seek(SeekFrom::Start(self.offset as u64))?;
        //Check if compression is used.
        if self.flags & 0x02 == 0 {
            //Uncompressed.
            let frame_size = self.width as u32 * self.height as u32;
            self.buffer = vec![0u8; frame_size as usize];
            reader.read_exact(&mut self.buffer)?;
        } else {
            //Compress using RLE.
            self.buffer = decompress_rle_data(reader, self.width, self.height)?;
        }
        debug_assert_eq!(
            self.buffer.len(),
            self.width as usize * self.height as usize
        );
        Ok(())
    }

    pub fn create_image(
        &self,
        palette: &Palette,
        width: u32,
        depth: u32,
    ) -> anyhow::Result<RgbaImage> {
        let mut image = RgbaImage::new(width, depth);
        let mut index = 0;
        for dy in 0..self.height {
            for dx in 0..self.width {
                let pixel = image.get_pixel_mut((self.x + dx) as u32, (self.y + dy) as u32);
                let color = self.buffer[index];
                if color == 0 {
                    *pixel = Rgba([0, 0, 0, 0]);
                } else {
                    *pixel = palette.get_color(color)?.into();
                }
                index += 1;
            }
        }
        Ok(image)
    }
}

fn decompress_rle_data<R: Read>(
    reader: &mut R,
    frame_width: u16,
    frame_height: u16,
) -> anyhow::Result<Vec<u8>> {
    let mut decompressed_data = Vec::with_capacity(frame_width as usize * frame_height as usize);
    for _ in 0..frame_height {
        let mut line_buffer = Vec::with_capacity(frame_width as usize);
        //Get the length of this line.
        let row_length = reader.read_u16::<LittleEndian>()?;
        //Two bytes of the line length have been read.
        let mut current_byte_index = 2;
        while current_byte_index < row_length {
            let control_byte = reader.read_u8()?;
            current_byte_index += 1;
            //0x00 represents transparent.
            if control_byte == 0x00 {
                //Number of transparent pixels.
                let transparent_count = reader.read_u8()?;
                current_byte_index += 1;
                line_buffer.extend(vec![0x00; transparent_count as usize]);
            } else {
                line_buffer.push(control_byte);
            }
        }
        //For unknown reasons, the line_buffer may be longer than the frame_width, in which case the excess part can be cut off.
        for index in 0..frame_width {
            let byte = line_buffer.get(index as usize).unwrap_or(&0);
            decompressed_data.push(*byte);
        }
    }
    Ok(decompressed_data)
}
