

use crate::data::data_saving::SaveData;
use super::company_manager::CompanyManager;
use super::super::id::ID;


/// Holds all a users shares of all the stocks
pub struct StockWallet {
    holders : Vec<StockHolder>,
}

/// Default stock wallet functions
impl StockWallet {

    /// Makes a new stock holder
    pub fn new() -> StockWallet {
        StockWallet {
            holders : Vec::new(),
        }
    }

    /// Gets the amount of stock in the wallet
    pub fn stock_amount(&self) -> usize {
        let mut stock_amount = 0;

        for holder in &self.holders {
            stock_amount += holder.stock_amount();
        }

        stock_amount
    }

    /// Adds a stock to the wallet
    pub fn add_stock(&mut self, stock : Stock) {
        //Checks if the wallet already has the stock contained
        let has_holder = self.get_stock_holder_by_id_mut(stock.company_id());

        match has_holder {
            // This will never error as we just checked that the IDs match :)
            Ok(holder) => holder.add_stock(stock).unwrap(),
            Err(_) => {
                //Makes a new stock holder
                let mut holder = StockHolder::new(stock.name().to_string(),stock.company_id());

                //Adds the stock to the holder (This can never fail as we just made the holder!)
                holder.add_stock(stock).unwrap();

                //Adds the holder to the wallet
                self.holders.push(holder);
            },
        }
    }

    /// Sells a certain amount of stock from a company
    /// Returns the amount of money made from selling
    pub fn sell_stock(&mut self, company_manager : &CompanyManager, company_id : ID, sell_amount : usize) -> Result<f32, String> {
        // Gets the holder of the stock
        let holder_result = self.get_stock_holder_by_id_mut(company_id);

        match holder_result {
            Err(error) => return Err(error),
            _ => (),
        };

        // Sells the stock
        holder_result.unwrap().sell_stock(company_manager, sell_amount)
    }

    /// Gets a stock holder by the companies ID
    fn get_stock_holder_by_id(&self, company_id : ID) -> Result<&StockHolder, String> {
        //Filters for all holders with the same ID
        let filtered : Vec<&StockHolder> = self.holders
            .iter()
            .filter(|holder|  holder.company_id.equals(company_id))
            .collect();

        //Checks that there is a stock holder with the ID
        if filtered.len() == 0 { return Err(String::from("No stock holder with the ID")); }
        if filtered.len() != 1 { return Err(String::from("Multiple stock holders with the same ID!")); }
        
        //Return the filtered stock holder
        Ok(filtered[0])
    }

    /// Gets a stock holder by the companies ID
    fn get_stock_holder_by_id_mut(&mut self, company_id : ID) -> Result<&mut StockHolder, String> {
        // Loops until it gets the holder with the companies ID
        for holder in self.holders.iter_mut() {
            if holder.company_id().equals(company_id) {
                return Ok(holder);
            }
        }
        // No holder was found
        Err(String::from("No stock with the company_ID found!"))
    }
}

/// Allows the stock to save data into a string
impl SaveData for StockWallet {
    /// Gets data in the form of a string from the stock
    fn get_data(&self) -> String {
        //Stock name
        let mut data : String = String::new();
        
        for holder in &self.holders {
            //Adds the holders data
            data.push_str(&holder.get_data());
            //Seperated by commas
            data.push('\n');
        }

        //Removes the extra ','
        if data.len() > 0 { data.pop(); }

        //Return the data
        data
    }
}

/// Prints the stock to the screen
impl std::fmt::Display for StockWallet {
    //Prints the stocks information when printed
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut text : String = String::new();

        for holder in &self.holders {
            //Adds the holders data
            text.push_str(&format!("\t{}", holder));
            //Seperated by commas
            text.push('\n');
        }

        //Removes the extra ','
        if text.len() > 0 { text.pop(); }

        write!(f, "Company Holder:\n{}", text)
    }
}

///Holds all a users shares of one stock
struct StockHolder {
    company_name : String,
    company_id : ID,
    average_purchase_price : f32,
    stock_amount : usize,
}

// Holds a stock
impl StockHolder {
    /// Makes a new stock holder from the company name and ID
    pub fn new(company_name : String, company_id : ID) -> StockHolder {
        StockHolder { 
            company_name, 
            company_id, 
            average_purchase_price: 0.0, 
            stock_amount: 0,
        }
    }

    /// Gets the company's name
    pub fn company_name(&self) ->&String {
        &self.company_name
    }

    /// Gets the company's ID
    pub fn company_id(&self) -> ID {
        self.company_id
    }

    /// Gets the average purchase price
    pub fn avg_purchase_price(&self) -> f32 {
        self.average_purchase_price
    }

    /// Gets the amount of stocks
    pub fn stock_amount(&self) -> usize {
        self.stock_amount
    }

    /// Adds a stock to the holder
    pub fn add_stock(&mut self, stock : Stock) -> Result<(), String> {
        //Checks that the company ID's match
        if !stock.company_id().equals(self.company_id) {
            return Err(String::from("Company IDs do not match"));
        }
        
        //Gets the current total price
        let total_price : f32 = stock.purchase_price + self.average_purchase_price * self.stock_amount as f32;
        
        //We now bought 1 stock
        self.stock_amount += 1;

        //Determines the new purchase price
        self.average_purchase_price = total_price / self.stock_amount() as f32;
        
        //Determines the new average stock price with the
        Ok(())
    }

    /// Sells the amount of stock from the handler
    /// Returns the amount of money made from selling
    pub fn sell_stock(&mut self, company_manager : &CompanyManager, sell_amount : usize) -> Result<f32, String> {
        // Ensures there is enough stock to sell
        if self.stock_amount() < sell_amount {
            return Err(String::from("Selling more stock than currently owned!"));
        }
        
        //Gets the company (So it know it's stock price!)
        let stock_price;

        match company_manager.get_company_by_id(self.company_id()) {
            Ok(company) => stock_price = company.stock_price(),
            Err(error) => return Err(error),
        }

        //Removes (x) number of stocks
        self.stock_amount -= sell_amount;
        //Returns how much money is made by selling the stock
        Ok(stock_price * sell_amount as f32)
    }
}

/// Allows the stock to save data into a string
impl SaveData for StockHolder {
    /// Gets data in the form of a string from the stock
    fn get_data(&self) -> String {
        //Stock name
        let mut data : String = String::new();
        
        // The amount of shares bought
        data.push_str(&self.stock_amount().to_string());
        data.push('_');

        // The company name
        data.push_str(self.company_name());
        data.push('_');

        //Stock purchase price
        data.push_str(&self.avg_purchase_price().to_string());

        //Return the data
        data
    }
}

/// Prints the stock to the screen
impl std::fmt::Display for StockHolder {
    //Prints the stocks information when printed
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Company ID: {}, Name: {}, Amount: {}, Average purchase price: {}$", self.company_id(), self.company_name(), self.stock_amount(), self.avg_purchase_price())
    }
}



/// A Stock represents a share of a company
/// In this program, they do not exist until a User buys one from a Company
/// They are destroyed when added to a StockWallet/Holder
pub struct Stock {
    company_id : ID,
    id : ID,
    name : String,
    purchase_price : f32,
}

/// Default stock options
impl Stock {
    /// Builds a new stock from the given parameters
    pub fn new(company_id : ID, name: String, purchase_price: f32) -> Stock {
        //Creates and returns the stock
        Stock {
            company_id,
            name,
            purchase_price,
            id : ID::new(),
        }
    }

    /// Getters
    
    /// Get the ID of the stock
    pub fn id(&self) -> ID {
        self.id
    }

    /// Gets the companies id, that owns this stock
    pub fn company_id(&self) -> ID {
        self.company_id
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
    /// None, if no company with such value exists
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

/// Allows the stock to save data into a string
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

/// Prints the stock to the screen
impl std::fmt::Display for Stock {
    //Prints the stocks information when printed
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stock ID: {}, Name: {}, Company ID: {}, Purchase Price: {}$", self.id().value(), self.name(), self.company_id(), self.purchase_price())
    }
}

