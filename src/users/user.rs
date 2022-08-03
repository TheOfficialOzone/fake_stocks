

//Needs to have Access to a stock list
use crate::companies::stock::{StockWallet, Stock};
use crate::CompanyManager;
use crate::id::ID;
use crate::SaveData;
use crate::users::password::Password;

/// A User can use their money to purchase stock in a company
pub struct User {
    id : ID,
    user_name : String,
    display_name : String,
    password : Password,   
    money : f32,
    stock_wallet : StockWallet,
}

/// Default User functions
impl User {
    /// Makes a new User
    pub fn new(user_name : String, display_name : String, password : Password) -> User {
        User {
            id : ID::new(),
            user_name,
            display_name, 
            password,
            money : 1000.0, 
            stock_wallet : StockWallet::new(),
        }
    }
    
    /// Gets the Users ID
    pub fn id(&self) -> ID {
        self.id
    }

    /// Gets the users User name
    pub fn user_name(&self) -> &String {
        &self.user_name
    }

    /// Get the display name from the user
    pub fn display_name(&self) -> &String {
        &self.display_name
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

    /// Attempts to enter the password to the User
    /// True on success, false if password is wrong
    pub fn try_password(&self, password : Password) -> bool {
        return self.password.compare(password);
    }

    /// Buys a stock
    pub fn buy_stock(&mut self, stock : Stock, buy_amount : usize) -> Result<(), String> {
        //Checks that the user has enough money to purchase the stock
        let total_cost = stock.purchase_price() * buy_amount as f32;
        if self.money() < total_cost { return Err(format!("{} does not have enough money to purchase {}", self, stock))}

        //Purchases the stock
        self.money -= total_cost;

        // Adds the stock to the wallet
        self.stock_wallet.add_stock(stock, buy_amount);
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
        let mut data : String = self.display_name().clone();
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
        write!(f, "User {}, Money: {}$, Stock amount: {} \n{}", self.display_name(), self.money(), self.stock_amount(), self.stocks_to_string())
    }
}


