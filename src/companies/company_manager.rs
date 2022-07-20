

use crate::Company;
use crate::SaveData;
use crate::ID;
use rand::Rng;

use super::stock;

/*
The Company manager holds all other companies
This is so you can search for specific companies by their IDs, etc
 */
pub struct CompanyManager<> {
    companies : Vec<Company>,
}

/*
Built in Company Manager functions
 */
impl CompanyManager {
    //Constructors
    /* 
    Creates a new company manager
    */
    pub fn new() -> CompanyManager {
        CompanyManager { 
            companies : Vec::new(),
        }
    }

    /// Getters

    /*
    Gets the companies from the Company Manager

    @return &Vec<Company>, A reference to the Company vector
     */
    pub fn companies(&self) -> &Vec<Company> {
        &self.companies
    }

    /*
    Gets the companies from the Company Manager as mutable

    @return &Vec<Company>, A reference to the Company vector
     */
    pub fn companies_mut(&mut self) -> &mut Vec<Company> {
        &mut self.companies
    }

    /// Gets a company
    pub fn get_company(&self, pos : usize) -> &Company {
        &self.companies[pos]
    }

    /// Gets a company mutably
    pub fn get_company_mut(&mut self, pos : usize) -> &mut Company {
        //Return the company at pos
        &mut self.companies[pos]
    }

    /// Gets a company by it's ID
    pub fn get_company_by_id(&self, id : &ID) -> Result<&Company, String> {
        //Checks every companies name
        let filtered : Vec<&Company> = self.companies()
            .iter()
            .filter(|company| company.id().equals(id))
            .collect();

        //Checks that there are results
        if filtered.len() == 0 { return Err(String::from("No company was found!")); }
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

    /// Setters

    /// Creates a new company in the manager
    pub fn new_company(&mut self, name : String, stock_price : f32) {
        let new_company = Company::new(name, stock_price);
        self.companies.push(new_company);
    }

   /// Updates the prices of the companies
    pub fn update(&mut self) {
        //Loops through each company
        for company in self.companies_mut() {
            let current_stock_price = company.stock_price();

            //Generates a random price change
            let mut rng = rand::thread_rng();
            let price_change : f32 = rng.gen_range(-5.0..5.0);

            let price_change_result = company.set_stock_price(current_stock_price + price_change);

            match price_change_result {
                Err(_error) => (),
                _ => (),
            }
        }
    }
}


impl SaveData for CompanyManager {
    /// Gets the Data of the Company manager in String form
    fn get_data(&self) -> String {
        let mut data : String = String::new();

        //Add each companies data
        for company in self.companies() {
            data.push_str(&company.get_data());
            data.push('\n');
        }

        //Return the data
        data
    }
}