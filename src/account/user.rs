

//Needs to have Access to a stock list
use crate::companies::stock::Stock;


/*
A User can use their money to purchase stock in a company
 */
pub struct User {
    name : String,
    money : f32,
    stocks : Vec<Stock>,
    stock_id_generator : u64,
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
            name, 
            money, 
            stocks: Vec::new(),
            stock_id_generator : 0,
        }
    }

    /// Getters

    //Get the name from the user
    pub fn name(&self) -> String {
        self.name.clone()
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
    pub fn assets_value(&self) -> f32 {
        //Defaults the total to our current money
        let mut total_value : f32 = self.money();

        //Adds each stocks value
        for stock in self.stocks.iter() {
            total_value += stock.purchase_price();
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



    //Gets the next ID for a stock
    fn get_next_stock_id(&mut self) -> u64 {
        self.stock_id_generator += 1;
        self.stock_id_generator
    }
    /// Setters    

    /// Buying Stock
    
    //Buys a stock
    pub fn buy_stock(&mut self, mut stock : Stock) -> Result<String, String> {
        //Checks that the user has enough money to purchase the stock
        if self.money() < stock.purchase_price() { return Err(format!("{} does not have enough money to purchase {}", self, stock))}

        //Stores the stocks name
        let stock_name = stock.name().clone();

        //Purchases the stock
        self.money -= stock.purchase_price();

        //Sets the stocks ID
        let stock_id_change = stock.set_id(self.get_next_stock_id());

        //Ensures the stocks ID was switched
        match stock_id_change {
            Err(error) => return Err(error),
            _ => (),
        }
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
Prints the User to the screen
 */
impl std::fmt::Display for User {
    //Prints the stocks information
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User {}, Money: {}$ \nStocks: \n{}", self.name(), self.money(), self.stocks_to_string())
    }
}


