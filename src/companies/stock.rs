

/*
A stock represents 
 */
pub struct Stock {
    company_id : u64,
    id : u64,
    name : String,
    purchase_price : f32,
}

/*
Default Stock functions
 */
impl Stock {
    /*
    Builds a new stock from the given parameters

    @param name, The name of the stock
    @param price, The price of the stock

    @return Stock, The newly created stock
    */
    pub fn new(company_id : u64,name: String, purchase_price: f32) -> Stock {
        //Creates and returns the stock
        Stock {
            company_id,
            name,
            purchase_price,
            id : 0,
        }
    }

    /// Getters
    
    //Get the ID of the stock
    pub fn id(&self) -> u64 {
        self.id
    }

    //Gets the companies id, that owns this stock
    pub fn company_id(&self) -> u64 {
        self.company_id
    }

    // Get the name of the stock
    pub fn name(&self) -> &String {
        &self.name
    }

    // Get the price of the stock
    pub fn purchase_price(&self) -> f32 {
        self.purchase_price
    }

    //Get the value of a stock
    pub fn value(&self) -> f32 {
        //Gets the companies price
        // match get_company_price(self.name()) {
        //     Ok(price) => return price,
        //     Err(err) => println!("Error: {}", err),
        // }
        //Returns 0.0 if there is an error
        0.0
    }

    //Sets the id of the stock
    pub fn set_id(&mut self, new_id : u64) -> Result<String, String> {
        //Ensures the ID has not already been set
        if self.id != 0 { return Err(String::from("ID has already been set!")); }
        //The new_id cannot be 0
        if new_id == 0 { return Err(String::from("ID cannot be set to 0")); }
        //Changes the ID
        self.id = new_id;
        
        Ok(format!("ID has been set to {}", self.id()))
    }
}

/*
Prints the stock to the screen
 */
impl std::fmt::Display for Stock {
    //Prints the stocks information when printed
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stock ID: {}, Name: {}, Company ID: {}, Purchase Price: {}$", self.id(), self.name(), self.company_id(), self.purchase_price())
    }
}

