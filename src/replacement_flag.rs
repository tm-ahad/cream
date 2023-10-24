use std::ops::Range;

pub struct SingleReplacementMap(pub Range<usize>, pub String);

impl SingleReplacementMap {
    pub fn new(range: Range<usize>, replace_with: String) -> Self {
        Self(range, replace_with)
    }
    pub fn to_tuple(self) -> (Range<usize>, String) {
        (self.0, self.1)
    }
}
