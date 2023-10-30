
pub struct ParsableFormat {
    pub script_starts: usize,
    pub temp_starts: usize,
    pub raw: String,
}

impl ParsableFormat {
    pub fn new(raw: String, temp_starts: usize) -> ParsableFormat {
        ParsableFormat {
            script_starts: 0,
            temp_starts,
            raw,
        }
    }
}

