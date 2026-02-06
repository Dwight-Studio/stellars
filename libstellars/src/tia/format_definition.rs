use crate::Color;
use crate::tia::colors::{NTSC_COLORS, PAL_COLORS, SECAM_COLORS};

#[derive(Clone)]
pub struct FormatDefinition {
    pub(crate) vblank: u8,
    pub(crate) full_frame: u16,
    pub(crate) total_counts: usize,
    pub(crate) framerate: f32,
    pub(crate) palette: [Color; 0x100],
}

// TODO: SCREEN_HEIGHT const should be moved to a field here for concistency
impl FormatDefinition {
    pub fn screen_height(&self) -> u16 {
        self.full_frame
    }

    pub fn framerate(&self) -> f32 {
        self.framerate
    }

    pub(crate) fn ntsc() -> FormatDefinition {
        Self {
            vblank: 40,
            full_frame: 262,
            total_counts: 262 * 228,
            framerate: 60.0,
            palette: NTSC_COLORS,
        }
    }

    pub(crate) fn pal() -> FormatDefinition {
        Self {
            vblank: 48,
            full_frame: 312,
            total_counts: 312 * 228,
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