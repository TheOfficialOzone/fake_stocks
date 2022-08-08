



use std::sync::atomic::{AtomicUsize, Ordering};

//IDs are unique for each one created
#[derive(Clone, Copy)]
pub struct ID {
    id : usize,
}

impl ID {
    /// Generates a new ID
    pub fn new<'a>() -> ID {
        static COUNTER : AtomicUsize = AtomicUsize::new(0);

        let prev_val = COUNTER.fetch_add(1, Ordering::Relaxed);
        ID { id: prev_val }
    }

    /// Force generated an ID
    fn new_forced_id(id_num : usize) -> ID {
        ID { id: id_num }
    }

    /// Gets the value of the ID
    pub fn value(&self) -> usize {
        self.id
    }

    /// Checks if two ID's are identical
    pub fn equals(&self, other : ID) -> bool {
        self.value() == other.value()
    }


    /// Makes an ID from a string
    /// Expects it in the form of "ID:654"
    pub fn from_string(str : &String) -> Result<ID, String> {
        //Checks that there is an ID
        if str.contains("ID=") && str.len() > 3  {
            let value_str = &str[3..].to_string();

            //Error parsing the ID
            match value_str.parse::<usize>() {
                Ok(value) => return Ok(Self::new_forced_id(value)),
                Err(error) => return Err(error.to_string()),
            };
        }
        //Could not find ID
        Err(String::from("No ID found"))
    }
}

/*
Prints the stock to the screen
 */
impl std::fmt::Display for ID {
    //Prints the stocks information when printed
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
