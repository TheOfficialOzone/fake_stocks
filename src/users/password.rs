




/// The Password struct stores a password (Cannot be changed once created)
/// A Password is a combination of 4 of the arrow keys 
#[derive(Clone, Copy)]
pub struct Password {
    keycodes : [u32; 4],
}

/// Default password functions
impl Password {
    /// Creates a new password from a set of keycodes
    pub fn new(keys : [u32; 4]) -> Password {
        Password { keycodes: keys }
    }

    /// Compares two passwords against one another
    pub fn compare(&self, other : Password) -> bool {
        self.keycodes == other.keycodes
    }
}