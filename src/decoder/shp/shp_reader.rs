use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

use byteorder::{LittleEndian, ReadBytesExt};
use godot::classes::{Texture2D, class_macros::private::virtuals::Xrvrs::Gd};

use crate::{decoder::pal::palette::Palette, errors::Ra2Error};

use super::shp_frame::ShpFrame;
use super::shp_header::ShpHeader;

pub struct ShpReader {
    header: ShpHeader,
    reader: BufReader<File>,
}

impl ShpReader {
    //Create a new ShpReader from a file.
    fn new(file_path: &Path) -> Result<Self, Ra2Error> {
        let file = File::open(file_path)?;

        let mut reader = BufReader::new(file);
        let shp_header = read_shp_header(&mut reader)?;

        Ok(Self {
            header: shp_header,
            reader,
        })
    }

    fn get_frame(&mut self, index: u64) -> Result<ShpFrame, Ra2Error> {
        self.reader.seek(SeekFrom::Start(8 + index * 24))?;
        let mut buffer = ShpFrame::default();
        buffer.read_shp_frame_header(&mut self.reader)?;
        buffer.read_shp_frame_data(&mut self.reader)?;
        Ok(buffer)
    }
}

fn read_shp_header<R: Read>(reader: &mut R) -> Result<ShpHeader, Ra2Error> {
    let reserved = reader.read_u16::<LittleEndian>()?;
    let width = reader.read_u16::<LittleEndian>()?;
    let height = reader.read_u16::<LittleEndian>()?;
    let number_of_frames = reader.read_u16::<LittleEndian>()?;
    Ok(ShpHeader {
        _reserved: reserved,
        width,
        height,
        number_of_frames,
    })
}

//Convert shp file to png format
pub fn decode_shp_to_image(
    shp_path: &Path,
    pal_path: &Path,
    is_half: bool,
) -> Result<Vec<Gd<Texture2D>>, Ra2Error> {
    let palette = Palette::load(pal_path)?;

    let mut handles = Vec::new();

    let mut shp = ShpReader::new(shp_path)?;

    match shp_path.extension() {
        Some(s) if s.eq("shp") => {
            //Shadow rendering toggle.
            let frame_count = if is_half {
                shp.header.number_of_frames / 2
            } else {
                shp.header.number_of_frames
            };

            for i in 0..frame_count {
                let frame = shp.get_frame(i as u64)?;
                let image = frame.create_image(
                    &palette,
                    shp.header.width as u32,
                    shp.header.height as u32,
                )?;

                handles.push(image);
            }
        }
        _ => {}
    }

    Ok(handles)
}
