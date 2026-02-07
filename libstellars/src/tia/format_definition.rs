use crate::Color;
use crate::tia::colors::{NTSC_COLORS, PAL_COLORS, SECAM_COLORS};

#[derive(Clone)]
pub struct FormatDefinition {
    pub(crate) vblank: u8,
    pub(crate) frame_height: u16,
    pub(crate) frame_width: u16,
    pub(crate) framerate: f32,
    pub(crate) palette: [Color; 0x100],
}

impl FormatDefinition {
    pub fn screen_height(&self) -> u16 {
        self.frame_height
    }
    pub fn screen_width(&self)  -> u16 {
        self.frame_width - 68
    }
    pub fn framerate(&self) -> f32 {
        self.framerate
    }

    pub(crate) fn ntsc() -> FormatDefinition {
        Self {
            vblank: 40,
            frame_height: 262,
            frame_width: 228,
            framerate: 60.0,
            palette: NTSC_COLORS,
        }
    }

    pub(crate) fn pal() -> FormatDefinition {
        Self {
            vblank: 48,
            frame_height: 312,
            frame_width: 228,
            framerate: 50.0,
            palette: PAL_COLORS,
        }
    }

    pub(crate) fn secam() -> FormatDefinition {
        let mut res = Self::pal();
        res.palette = SECAM_COLORS;
        
        res
    }
}