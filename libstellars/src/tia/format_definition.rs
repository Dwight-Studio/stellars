#[derive(Clone)]
pub struct FormatDefinition {
    pub(crate) vblank: u8,
    pub(crate) full_frame: u16,
    pub(crate) total_counts: usize,
    pub(crate) framerate: f32,
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
            total_counts: 262 * 228 - 1000, // FIXME: Some current emulation issued might be caused by this hack... We shouldn't subtract anything here.
            framerate: 60.0,
        }
    }

    pub(crate) fn pal() -> FormatDefinition {
        Self {
            vblank: 48,
            full_frame: 312,
            total_counts: 312 * 228 - 1000,
            framerate: 50.0,
        }
    }

    pub(crate) fn secam() -> FormatDefinition {
        Self::pal()
    }
}