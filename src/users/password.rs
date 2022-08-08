




/// The Password struct stores a password (Cannot be changed once created)
/// A Password is a combination of 4 of the arrow keys 
#[derive(Clone, Copy)]
pub struct Password {
    keycodes : [u16; 6],
}

/// Default password functions
impl Password {
    /// Creates a new password from a set of keycodes
    pub fn new(keys : [u16; 6]) -> Password {
        Password { keycodes: keys }
    }

    /// Parses a password from text
    pub fn from_text(password : &String) -> Result<Password, String> {
        //Defaults all keycodes to 0
        let mut keycodes : [u16; 6] = [0 ; 6];

        //Splits the password by '-'
        let split_password : Vec<&str> = password.split("-").collect();
        //Ensures the length is 6
        if split_password.len() != 6 { return Err(String::from("Length of password is not 6!"))}

        let mut pos : usize = 0;
        //Loops through each bit
        for code in split_password {
            match code {
                "left" => keycodes[pos] = 1,
                "right" => keycodes[pos] = 2,
                "up" => keycodes[pos] = 3,
                "down" => keycodes[pos] = 4,
                _ => return Err(format!("{} is not a valid password entry!", code)),
            }
            //adds to the size
            pos += 1;
        };

        //Create the password from the text
        Ok(Self::new(keycodes))
    } 

    /// Compares two passwords against one another
    pub fn compare(&self, other : Password) -> bool {
        self.keycodes == other.keycodes
    }
}