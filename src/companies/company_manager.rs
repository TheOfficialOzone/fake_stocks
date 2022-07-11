

use crate::Company;

/*
The Company manager holds all other companies
This is so you can search for specific companies by their IDs, etc
 */
pub struct CompanyManager<> {
    companies : Vec<Company>,
    id_generator : u64,
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
            id_generator : 1,
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
    Gets the next ID from the Company manager

    @return u64, The next ID to use
     */
    fn get_next_id(&mut self) -> u64 {
        //Increase the id_generator by 1, before returning
        self.id_generator += 1;
        self.id_generator
    }


    /*
    Gets the company at [pos] position

    @param pos, The position of the desired company

    @return Result<&Company, String>, The Company at that position
     */
    pub fn get_company(&self, pos : usize) -> Result<&Company, String> {
        //Checks if the position is valid
        if self.companies.len() < pos { return Err(format!("Position out of bounds : {}", pos))}
        //Return the company at pos
        Ok(&self.companies[pos])
    }

    /*
    Gets the company at [pos] position, mutably

    @param pos, The position of the desired company

    @return Result<&Company, String>, The Company at that position
     */
    pub fn get_company_mut(&mut self, pos : usize) -> Result<&mut Company, String> {
        //Checks if the position is valid
        if self.companies.len() < pos { return Err(format!("Position out of bounds : {}", pos))}
        //Return the company at pos
        Ok(&mut self.companies[pos])
    }

    /*
    Gets a company by the ID

    @param id, The id of the company

    @return Result<&Company, String>, The company with said ID
     */
    pub fn get_company_by_id(&self, id : u64) -> Result<&Company, String> {
        //Checks every companies name
        let filtered : Vec<&Company> = self.companies()
            .iter()
            .filter(|company| company.id() == id)
            .collect();

        //Checks that there are results
        if filtered.len() == 0 { return Err(String::from("No company was found!")); }
        //Ensures there is only 1 result
        if filtered.len() != 1 { return Err(String::from("Multiple companies with the same name exist!"))}

        //Return the only company
        Ok(&filtered[0])
    }

    /*
    Gets a company by it's name

    @param name, The name of the company

    @return Result<&Company, String>, The company with said name
     */
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

    /*
    Adds a company to the managers list

    @param company, The company to add

    @return Result<String, String>, Ok() is a success message, Err() is an error message 
    */
    pub fn add_company(&mut self, mut company : Company) -> Result<String, String> {
        //Attempts to set the companies ID
        let change_id = company.set_id(self.get_next_id());
        
        //Checks if there was an error changing the ID
        match change_id {
            Err(error) => return Err(format!("Failed adding company due to error: {}", error)),
            _ => (),
        }
        //Stores the companies name
        let held_name = company.name().clone();
        //Adds the company to the vector
        self.companies.push(company);
        //Prints out the company
        Ok(format!("Added company: {}", held_name))
    }
}



// Old Garbage that im unsure if to delete

    // /*
    // Gets the company's price from it's name

    // @param name, The name of the company

    // @return Result<f32, String>, Ok() is the companies price, Err() is an error message 
    // */
    // pub fn get_company_price_by_name(&self, name : &String) -> Result<f32, String> {
    //     //Gets the company
    //     let result = self.get_company_by_name(name);

    //     //Checks the result
    //     match result {
    //         Err(err) => return Err(err),
    //         _ => (),
    //     }

    //     //Returns the price of the company
    //     Ok(result.unwrap().stock_price())
    // }

    // /*
    // Gets the company's price from it's id

    // @param id, The id of the company

    // @return Result<f32, String>, Ok() is the companies price, Err() is an error message 
    // */
    // pub fn get_company_price_by_id(&self, id : u64) -> Result<f32, String> {
    //     //Gets the company
    //     let result = self.get_company_by_id(id);

    //     //Checks the result
    //     match result {
    //         Err(err) => return Err(err),
    //         _ => (),
    //     }

    //     //Returns the price of the company
    //     Ok(result.unwrap().stock_price())
    // }