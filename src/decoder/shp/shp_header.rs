pub struct ShpHeader {
    //Unused reserved header.
    pub _reserved: u16,
    //The animation width.
    pub width: u16,
    //The animation height.
    pub height: u16,
    //The animation frames.
    pub number_of_frames: u16,
}
