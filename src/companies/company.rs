


use crate::{Stock, User, data::data_saving::SaveData};
use crate::ID;

/*
A Company is similar to a real life company

They have shares that can be bought and sold at the price
They are responsible for tracking their previous stock price
 */
pub struct Company {
    name : String,
    id : ID,
    stock_price : f32,
    stock_price_history : Vec<f32>,
}

/*
Default Company functions
 */
impl Company {
    /*
    Builds a new company from the given parameters

    @param name, The name of the stock
    @param stock_amount, The amount of stock
    @param stock_price, The price of the stock

    @return Company, The newly created Company
    */
    pub fn new(name : String, stock_price : f32) -> Company {
        Company {
            name,
            id : ID::new(),
            stock_price,
            stock_price_history : vec!(stock_price), // (Starts the pricing history at the current price)
        }
    }

    /// Getters

    //Get the name of the company
    pub fn name(&self) -> &String {
        &self.name
    }

    //Gets the ID of the company
    pub fn id(&self) -> &ID {
        &self.id
    }

    //Get the current price of the stock
    pub fn stock_price(&self) -> f32 {
        self.stock_price
    }

    //Gets the stock price history of the company
    pub fn stock_price_history(&self) -> &Vec<f32> {
        &self.stock_price_history
    }

    /// Setters

    //Sets a new price for the Company
    pub fn set_stock_price(&mut self, new_price : f32) -> Result<String, String>{
        if new_price < 0.0 { return Err(String::from("Price cannot be set to a negative value!")); }

        //Sets the new price
        self.stock_price = new_price;
        
        //Save the prices in the company history
        self.stock_price_history.push(new_price);

        //Returns the valid result
        Ok(format!("Price of {} was set to {}", self.name(), new_price))
    }

    /// Purchasing stock
    
    //Purchases a stock from the company
    pub fn purchase_stock(&self, user : &mut User) -> Result<String, String> {
        //Creates the bought stock
        let stock = Stock::new(self.id(), self.name.clone(), self.stock_price());

        //Returns the result of the users buy
        user.buy_stock(stock)
    }

}

impl SaveData for Company {
    fn get_data(&self) -> String {
        //Starts with the name of the company
        let mut data : String = self.name().clone();

        //Write each value of the history
        for value in self.stock_price_history().iter() {
            data.push(',');
            data.push_str(&value.to_string());
        }
        //Return the data
        data
    }
}

/*
Prints the company to the screen
 */
impl std::fmt::Display for Company {
    //Prints the stocks information when printed
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Company {}, Stock price: {}$", self.name(), self.stock_price())
    }
}

