



use std::sync::atomic::{AtomicU64, Ordering};

//IDs are unique for each one created
#[derive(Clone, Copy)]
pub struct ID {
    id : u64,
}

impl ID {
    //Generates a new ID
    pub fn new<'a>() -> ID {
        static COUNTER : AtomicU64 = AtomicU64::new(0);

        let prev_val = COUNTER.fetch_add(1, Ordering::Relaxed);
        ID { id: prev_val }
    }

    //Gets the value of the ID
    pub fn value(&self) -> u64 {
        self.id
    }

    //Checks if two ID's are identical
    pub fn equals(&self, other : ID) -> bool {
        self.value() == other.value()
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
