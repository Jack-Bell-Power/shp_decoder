use std::{
    fs::File, io::{BufReader, Read, Seek, SeekFrom}, path::Path,
};

use byteorder::{LittleEndian, ReadBytesExt};
use image::RgbaImage;

use crate::decoder::pal::palette::Palette;

use super::shp_frame::ShpFrame;
use super::shp_header::ShpHeader;

pub struct ShpReader {
    header: ShpHeader,
    reader: BufReader<File>,
}

impl ShpReader {
    //Create a new ShpReader from a file.
    fn new(file_path: &Path) -> anyhow::Result<Self> {
        let file = File::open(file_path)?;

        let mut reader = BufReader::new(file);
        let shp_header = read_shp_header(&mut reader)?;

        Ok(Self {
            header: shp_header,
            reader,
        })
    }

    fn get_frame(&mut self, index: u64) -> anyhow::Result<ShpFrame> {
        self.reader.seek(SeekFrom::Start(8 + index * 24))?;
        let mut buffer = ShpFrame::default();
        buffer.read_shp_frame_header(&mut self.reader)?;
        buffer.read_shp_frame_data(&mut self.reader)?;
        Ok(buffer)
    }
}

fn read_shp_header<R: Read>(reader: &mut R) -> anyhow::Result<ShpHeader> {
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

//Convert shp file to rgba image format
fn decode_shp_to_rgba_image(
    shp_path: &Path,
    pal_path: &Path,
    is_half: bool,
) -> anyhow::Result<Vec<RgbaImage>> {
    let palette = Palette::load(pal_path)?;

    let mut iamges = Vec::new();

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

                iamges.push(image);
            }
        }
        _ => {}
    }

    Ok(iamges)
}

pub fn rgba_image_to_png(
    shp_path: &Path,
    pal_path: &Path,
    is_half: bool,
    output_path: &Path,
) -> anyhow::Result<()> {
    let images = decode_shp_to_rgba_image(shp_path, pal_path, is_half)?;
    let name = shp_path.file_stem().unwrap().to_string_lossy();
    // Make sure the path exists.
    std::fs::create_dir_all(output_path)?;

    for (i, image) in images.iter().enumerate() {
        let file_path = output_path.join(
            format!("{}_{:03}.png", name, i)
        );
        image.save(file_path)?;
    }

    Ok(())
}
