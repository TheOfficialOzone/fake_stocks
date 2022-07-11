


use crate::companies::company::Company;
use crate::companies::company_manager::CompanyManager;
use crate::companies::stock::Stock;
use crate::account::user::User;


use std::time::Instant;

//So it can use accounts and companies
mod companies;
mod account;


fn main() {
    let now = Instant::now();

    let mut company_manager : CompanyManager = CompanyManager::new();
    //Create amazon

    //Create Jeff Bezos
    let mut jeffy : User = User::new(String::from("Jeffry Bezos"), 1000.0);

    let _ = company_manager.add_company(Company::new(String::from("Not Amazon"), 10, 10.0));
    let amazon = company_manager.get_company_mut(0).unwrap();

    println!("{}", jeffy);

    let purchase_result = amazon.purchase_stock(&mut jeffy);

    match purchase_result {
        Err(error) => println!("{}", error),
        Ok(success) => println!("{}", success),
    }

    amazon.set_stock_price(69.00);

    println!("{}", amazon);

    println!("{}", jeffy);

    println!("Time Elapsed: {}", now.elapsed().as_millis());
}
