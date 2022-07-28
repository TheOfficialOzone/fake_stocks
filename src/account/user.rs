

//Needs to have Access to a stock list
use crate::companies::stock::{StockWallet, Stock};
use crate::CompanyManager;
use crate::id::ID;
use crate::SaveData;

/// A User can use their money to purchase stock in a company
pub struct User {
    id : ID,
    name : String,
    money : f32,
    stock_wallet : StockWallet,
}

/// Default User functions
impl User {

    /// Makes a new User
    pub fn new(name : String, money : f32) -> User {
        User {
            id : ID::new(),
            name, 
            money, 
            stock_wallet : StockWallet::new(),
        }
    }

    /// Getters
    
    /// Gets the Users ID
    pub fn id(&self) -> ID {
        self.id
    }

    /// Get the name from the user
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the amount of money the user has
    pub fn money(&self) -> f32 {
        self.money
    }

    /// Gets the stock wallet from the user
    pub fn wallet(&self) -> &StockWallet {
        &self.stock_wallet
    }

    /// Gets the amount of stock the user has
    pub fn stock_amount(&self) -> usize {
        self.wallet().stock_amount()
    }

    /// Setters    

    /// Buying Stock
    
    /// Buys a stock
    pub fn buy_stock(&mut self, stock : Stock) -> Result<(), String> {
        //Checks that the user has enough money to purchase the stock
        if self.money() < stock.purchase_price() { return Err(format!("{} does not have enough money to purchase {}", self, stock))}

        //Purchases the stock
        self.money -= stock.purchase_price();

        // Adds the stock to the wallet
        self.stock_wallet.add_stock(stock);
        Ok(())
    }

    /// Sells stock stock from the user
    pub fn sell_stock(&mut self, company_manager : &CompanyManager, company_id : ID, sell_amount : usize) -> Result<(), String> {
        //Sells the stock from the stock wallet
        let sell_result = self.stock_wallet.sell_stock(company_manager, company_id, sell_amount);

        match sell_result {
            Ok(sell_money) => self.money += sell_money,
            Err(error) => return Err(error),
        }

        Ok(())
    }

    /// Gets all the stocks of the user into a string
    fn stocks_to_string(&self) -> String {
        let mut stock_string : String = String::new();
        
        stock_string.push_str(&format!("{}", self.wallet()));

        stock_string
    }
}

/// Allows the user to save data
impl SaveData for User {
    /// Gets the Users data
    fn get_data(&self) -> String {
        //Starts with the name of the user
        let mut data : String = self.name().clone();
        data.push('\n');

        //Write the wallet into the data
        data.push_str(&self.wallet().get_data());

        //Return the data
        data
    }
}


 /// Prints the User to the screen
impl std::fmt::Display for User {
    /// Prints the Users information
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User {}, Money: {}$, Stock amount: {} \n{}", self.name(), self.money(), self.stock_amount(), self.stocks_to_string())
    }
}


