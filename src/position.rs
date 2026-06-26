#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub rank: usize, // between 0 and 7
    pub file: usize, // between 0 and 7
}

impl Position {
    pub fn index(self) -> usize {
        self.rank * 8 + self.file
    }

    pub fn from_index(index: usize) -> Option<Self> {
        if index < 64 {
            Some(Self {
                rank: index / 8,
                file: index % 8,
            })
        } else {
            None
        }
    }
}
