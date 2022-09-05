

use crate::Company;
use crate::SaveData;
use crate::ID;
use rand::Rng;


/// The Company manager holds all other companies
/// This is so you can search for specific companies by their IDs, etc
#[derive(Debug)]
pub struct CompanyManager<> {
    companies : Vec<Company>,
    stored_save : String,
}



/*
Built in Company Manager functions
 */
impl CompanyManager {
    /// Creates a new Company Manager
    pub fn new() -> CompanyManager {
        CompanyManager { 
            companies : Vec::new(),
            stored_save : String::new(),
        }
    }

    /// Creates a new company in the manager
    pub fn new_company(&mut self, name : String, stock_price : f32) -> ID {
        //Create the new company
        let new_company = Company::new(name, stock_price);
        //Copy the ID
        let company_id = new_company.id();
        self.companies.push(new_company);

        company_id
    }

    /// Gets the Company list
    pub fn companies(&self) -> &Vec<Company> {
        &self.companies
    }

    /// Gets the company list mutably
    pub fn companies_mut(&mut self) -> &mut Vec<Company> {
        &mut self.companies
    }

    /// Gets a company by it's ID
    pub fn get_company_by_id(&self, id : ID) -> Result<&Company, String> {
        //Checks every companies name
        let filtered : Vec<&Company> = self.companies()
            .iter()
            .filter(|company| company.id().equals(id))
            .collect();

        //Checks that there are results
        if filtered.len() == 0 { return Err(format!("No company with ID {} was found!", id)); }
        //Ensures there is only 1 result
        if filtered.len() != 1 { return Err(String::from("Multiple companies with the same name exist!"))}

        //Return the only company
        Ok(&filtered[0])
    }

    /// Gets a company by it's name
    pub fn get_company_by_name(&self, name : &String) -> Result<&Company, String> {
        //Checks every companies name
        let filtered : Vec<&Company> = self.companies()
            .iter()
            .filter(|company| company.name().eq(name))
            .collect();

        //Checks that there are results
        if filtered.len() == 0 { return Err(String::from("No company was found!")); }
        //Ensures there is only 1 result
        if filtered.len() != 1 { return Err(String::from("Multiple companies with the same name exist!"))}

        //Return the only company
        Ok(&filtered[0])
    }

    /// Gets a company by it's name
    pub fn get_company_by_name_mut(&mut self, name : &String) -> Result<&mut Company, String> {
        for company in self.companies_mut() {
            if company.name().eq(name) {
                return Ok(company);
            }
        }
        Err(format!("No Company with name {} found", name))
    }

    /// Updates the prices of the companies
    pub fn update(&mut self) {
        const STOCK_RANGE : f32 = 20.0;
        //Loops through each company
        for company in self.companies_mut() {
            let current_stock_price = company.stock_price();

            //Generates a random price change
            let mut rng = rand::thread_rng();
            let price_change : f32 = rng.gen_range(-STOCK_RANGE..STOCK_RANGE);

            let price_change_result = company.set_stock_price(current_stock_price + price_change);

            match price_change_result {
                Err(_error) => (),
                _ => (),
            }
        }

        //Updates the stored save data
        self.stored_save.clear();
        self.stored_save = self.get_data();
    }
}



impl SaveData for CompanyManager {
    /// Gets the Data of the Company manager in String form
    fn get_data(&self) -> String {
        //If the stored save data isn't empty, return it as it has yet to be updated
        if !self.stored_save.is_empty() {
            return self.stored_save.clone();
        }

        let mut data : String = String::new();

        //Add each companies data
        for company in self.companies() {
            //Adds the companies data
            data.push_str(&company.get_data());
            data.push('\n');
        }
        //removes the last '\n'
        if data.len() > 0 {
            data.pop();
        }
        //Return the data
        data
    }
}


/// Prints the Company Manager
impl std::fmt::Display for CompanyManager {
    //Prints the stocks information
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut text = String::new();

        text.push_str("--- Company Manager ---\n");
        //Loops through every company manager
        for company in self.companies() {
            text.push_str(&format!("\t{}\n", company));
        }

        write!(f, "{}", text)
    }
}
