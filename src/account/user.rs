

//Needs to have Access to a stock list
use crate::companies::{stock::Stock, company_manager::CompanyManager};
use crate::id::ID;
use crate::SaveData;

/*
A User can use their money to purchase stock in a company
 */
pub struct User {
    id : ID,
    name : String,
    money : f32,
    stocks : Vec<Stock>,
}


/*
Default User functions
 */
impl User {
    /*
    builds a user who can now buy stock

    @param name, The name of the User
    @param moeny, The starting cash the user will have

    @return User, The newly built user
    */
    pub fn new(name : String, money : f32) -> User {
        User {
            id : ID::new(),
            name, 
            money, 
            stocks: Vec::new(),
        }
    }

    /// Getters
    
    //Gets the Users ID
    pub fn id(&self) -> &ID {
        &self.id
    }

    //Get the name from the user
    pub fn name(&self) -> &String {
        &self.name
    }

    //Get the amount of money the user has
    pub fn money(&self) -> f32 {
        self.money
    }

    //Gets the stocks from the user
    pub fn stocks(&self) ->&Vec<Stock> {
        &self.stocks
    }

    //Gets the total amount of money a user has (In money and stock combined!)
    pub fn assets_value(&self, company_manager : &CompanyManager) -> f32 {
        //Defaults the total to our current money
        let mut total_value : f32 = self.money();

        //Adds each stocks value
        for stock in self.stocks.iter() {
            let value = stock.value(company_manager);
            match value {
                Some(x) => total_value += x,
                _ => (),
            }
        }

        total_value
    }

    //Gets the amount of stock the user has
    pub fn stock_amount(&self) -> usize {
        self.stocks.len()
    }

    //Gets a stock from the user
    pub fn get_stock(&self, pos : usize) -> &Stock {
        //Gets the stock at that position
        &self.stocks[pos]
    }

    /// Setters    

    /// Buying Stock
    
    //Buys a stock
    pub fn buy_stock(&mut self, stock : Stock) -> Result<String, String> {
        //Checks that the user has enough money to purchase the stock
        if self.money() < stock.purchase_price() { return Err(format!("{} does not have enough money to purchase {}", self, stock))}

        //Stores the stocks name
        let stock_name = stock.name().clone();

        //Purchases the stock
        self.money -= stock.purchase_price();

        //Adds the stock to the vector
        self.stocks.push(stock);

        Ok(format!("Bought stock: {}", stock_name))
    }

    /*
    Gets all the stocks of the user into one string
     */
    pub fn stocks_to_string(&self) -> String {
        let mut stock_string : String = String::new();

        for stock in self.stocks() {
            stock_string.push_str(&format!("\t{}\n", stock));
        }

        stock_string
    }
}

/*
Allows the user to Save Data
 */
impl SaveData for User {
    fn get_data(&self) -> String {
        //Starts with the name of the company
        let mut data : String = self.name().clone();

        //Write each stock in the list 
        //Format [NAME]:[PURCHASE_PRICE]
        for stock in self.stocks.iter() {
            data.push(',');
            //Stocks name
            data.push_str(stock.name());
            data.push('_');
            data.push_str(&stock.purchase_price().to_string());
        }
        //Return the data
        data
    }
}


/*
Prints the User to the screen
 */
impl std::fmt::Display for User {
    //Prints the stocks information
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User {}, Money: {}$ \nStocks: \n{}", self.name(), self.money(), self.stocks_to_string())
    }
}


