

use crate::{companies::company_manager::CompanyManager, data::data_saving::SaveData};
use super::{user::User, user_manager::UserManager};


/// Holds the ranking of a user
#[derive(Debug)]
struct Rank {
    name : String,
    value : f32,
}


/// Default Rank functions
impl Rank {
    /// Makes a new rank
    fn new(name : String, value : f32) -> Rank {
        Rank { name, value }
    }

    /// Makes a rank from a user and the company manager
    pub fn rank_from_user(user : &User, company_manager : &CompanyManager) -> Result<Rank, String> {
        match user.value(company_manager) {
            Ok(value) => Ok(Self::new(user.user_name().clone(), value)),
            Err(error) => Err(error),
        }
    }
}

/// Prints the Rank to the screen
impl std::fmt::Display for Rank {
    /// Prints the Users information
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User {}, Value: {}$", self.name, self.value)
    }
}

/// Save Data
impl SaveData for Rank {
    /// Saves the data of the Rank
    fn get_data(&self) -> String {
        format!("{}_{}", self.name, self.value)
    }
}

/// Ranks all users against one-another to determine
pub struct Ranker {
    order : Vec<Rank>,
}

/// Default Ranker functions
impl Ranker {
    /// Makes a new Ranker
    pub fn new() -> Ranker {
        Ranker { order : Vec::new() }
    }

    //Ranks all the users 
    pub fn rank_users(&mut self, user_manager : &UserManager, company_manager : &CompanyManager) -> Result<(), String> {
        //Reset the rankings
        self.order.clear();
        
        //Loop through every user
        for user in user_manager.users() {
            //Make a rank from each user
            match Rank::rank_from_user(user, company_manager) {
                Ok(new_rank) => self.order.push(new_rank),
                Err(error) => return Err(error),
            }
        }

        println!("Current order:\n{:?}", self.order);
        Ok(())
    }


    /// Gets the ranks in string to send over the server
    /// Gets the ranks from the range specified
    pub fn get_data_range(&self, range : std::ops::Range<usize>) -> Result<String, String> {
        // Keeps the range in the vectors bounds
        if range.start >= self.order.len() { return Err(format!("Start of range is out of bounds {}", range.start)); }
        if range.end > self.order.len() { return Err(format!("End of range is out of bounds {}", range.end)); }

        //Bounds the range to the size of the array
        let mut data = String::new();
        //Loops through each rank in the range
        for rank in &self.order[range] {
            data.push_str(&rank.get_data());
            data.push(',');
        };

        //Removes the extra data
        if data.len() != 0 { data.pop(); }

        Ok(data)
    }
}

