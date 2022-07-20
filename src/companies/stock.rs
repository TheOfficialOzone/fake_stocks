use crate::data::data_saving::SaveData;

use super::company_manager::CompanyManager;



use super::super::id::ID;

/// A Stock represents a share of a company
/// In this program, they do not exist until a User buys one from a Company
pub struct Stock {
    company_id : ID,
    id : ID,
    name : String,
    purchase_price : f32,
}

/// Default stock options
impl Stock {
    /// Builds a new stock from the given parameters
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
    
    /// Get the ID of the stock
    pub fn id(&self) -> &ID {
        &self.id
    }

    /// Gets the companies id, that owns this stock
    pub fn company_id(&self) -> &ID {
        &self.company_id
    }

    /// Get the name of the stock
    pub fn name(&self) -> &String {
        &self.name
    }

    ///  Get the price of the stock
    pub fn purchase_price(&self) -> f32 {
        self.purchase_price
    }

    /// Get the value of a stock
    ///  None, if no company with such value exists
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


impl SaveData for Stock {
    /// Gets data in the form of a string from the stock
    fn get_data(&self) -> String {
        //Stock name
        let mut data : String = self.name().clone();

        //Stock purchase price
        data.push(',');
        data.push_str(&self.purchase_price().to_string());

        data
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

