


use crate::{Stock, data::data_saving::SaveData};
use crate::User;
use crate::ID;


/// A Company is similar to a real life company
/// 
/// They have shares that can be bought and sold at the price
/// They are responsible for tracking their previous stock price
#[derive(Debug)]
pub struct Company {
    id : ID,
    name : String,
    stock_price : f32,
    stock_price_history : Vec<f32>,
}


/// Default COmpany functions
impl Company {
    /// Builds a new company from the given parameters
    pub fn new(name : String, stock_price : f32) -> Company {
        Company {
            name,
            id : ID::new(),
            stock_price,
            stock_price_history : vec!(stock_price), // (Starts the pricing history at the current price)
        }
    }

    /// Getters

    /// Get the name of the company
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Gets the ID of the company
    pub fn id(&self) -> ID {
        self.id
    }

    /// Get the current price of the stock
    pub fn stock_price(&self) -> f32 {
        self.stock_price
    }

    /// Gets the stock price history of the company
    pub fn stock_price_history(&self) -> &Vec<f32> {
        &self.stock_price_history
    }

    /// Sets a new price for the Company
    pub fn set_stock_price(&mut self, new_price : f32) -> Result<(), String>{
        //Ensures the new price is valid
        if new_price < 0.0 { return Err(String::from("Price cannot be set to a negative value!")); }

        //Sets the new price
        self.stock_price = new_price;
        
        //Save the prices in the company history
        self.stock_price_history.push(new_price);

        //Returns the valid result
        Ok(())
    }

    /// Resets the companies stock history
    pub fn reset_company(&mut self, new_price : f32) -> Result<(), String> {
        //Ensures the new price is valid
        if new_price < 0.0 { return Err(String::from("Price cannot be set to a negative value!")); }
        //Clears the stock history
        self.stock_price_history.clear();
        //Sets the new price
        self.set_stock_price(new_price)
    }

    /// Purchases a stock from the company
    pub fn purchase_stock(&self, user : &mut User, buy_amount : usize) -> Result<(), String> {
        //Creates the bought stock
        let stock = Stock::new(self.id(), self.name.clone(), self.stock_price());

        //Returns the result of the users buy
        user.buy_stock(stock, buy_amount)
    }

}

impl SaveData for Company {
    /// Saves the companies data to a string
    fn get_data(&self) -> String {
        //Starts with the name of the company
        let mut data : String = self.name().clone();

        //IF the length is less than 50
        if self.stock_price_history().len() < 50 {
            //Write each value of the history
            for value in self.stock_price_history().iter() {
                data.push(',');
                data.push_str(&value.to_string());
            }
        } else {
            let price_history = self.stock_price_history();
            for value in &price_history[price_history.len()-50..price_history.len()] {
                data.push(',');
                data.push_str(&value.to_string());
            }
        }
        //Return the data
        data
    }
}


/// Prints the company to the screen
impl std::fmt::Display for Company {
    /// Prints the stocks information
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Company {}, Stock price: {}$", self.name(), self.stock_price())
    }
}

