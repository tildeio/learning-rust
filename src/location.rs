// We want to be able to debug Location, `==` it (which requires
// Eq and PartialEq), use it as a HashMap key, and clone it.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Location {
    pub x: u64, // it has an x and y, both of which are unsigned numbers
    pub y: u64, // that means they cannot be negative (and we'll get an
}           // error if we try to underflow)

// The implementation is separate from the struct definition, which
// just describes the data structure. You can have as many `impl`
// blocks as you want, and they get merged together.
impl Location {
    // This provides Location::new(), which takes two `u64`s and
    // produces a `Location` object.
    pub fn new(x: u64, y: u64) -> Location {
        // Note that the lack of a `self` parameter is what makes
        // this a "class method", which means it is invoked as
        // Location::new(x, y).
        Location { x: x, y: y }
    }
}