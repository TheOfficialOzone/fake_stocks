use super::company_manager::CompanyManager;


use super::super::id::ID;

/*
A stock represents a share of a company
They do not exist until a User buys one from the company
 */
pub struct Stock {
    company_id : ID,
    id : ID,
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
    pub fn new(company_id : &ID, name: String, purchase_price: f32) -> Stock {
        //Creates and returns the stock
        Stock {
            company_id : company_id.clone(),
            name,
            purchase_price,
            id : ID::new(),
        }
    }

    /// Getters
    
    //Get the ID of the stock
    pub fn id(&self) -> &ID {
        &self.id
    }

    //Gets the companies id, that owns this stock
    pub fn company_id(&self) -> &ID {
        &self.company_id
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
    // None, if no company with such value exists
    pub fn value(&self, company_manager : &CompanyManager) -> Option<f32> {
        //Gets the company by it's ID
        let company_price = company_manager.get_company_by_id(self.company_id());

        //Gets the companies price
        match company_price {
            Err(_) => return None,
            Ok(company) => return Some(company.stock_price()),
        }
    }
}

/*
Prints the stock to the screen
 */
impl std::fmt::Display for Stock {
    //Prints the stocks information when printed
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stock ID: {}, Name: {}, Company ID: {}, Purchase Price: {}$", self.id().value(), self.name(), self.company_id(), self.purchase_price())
    }
}

