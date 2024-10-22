extern crate rand;
extern crate paste;

struct Die {
    faces: u8
}
impl Die {
    pub fn new(faces: u8) -> Die {
        Die { faces }
    }
    pub fn d2() -> Die {
        Self::new(2)
    }
    pub fn d4() -> Die {
        Self::new(4)
    }
    pub fn d6() -> Die {
        Self::new(6)
    }
    // Many more functions for other dice
}


fn main() {

}