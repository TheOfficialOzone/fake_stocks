

use std::cmp::Ordering;

use crate::{companies::company_manager::CompanyManager, data::data_saving::SaveData};
use super::{user::User, user_manager::UserManager};


/// Holds the ranking of a user
#[derive(Clone, Debug)]
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
            Ok(value) => Ok(Self::new(user.display_name().clone(), value)),
            Err(error) => Err(error),
        }
    }

    /// The standard comparason function
    /// 
    pub fn cmp(&self, other : &Rank) -> std::cmp::Ordering {
        //Compares all the values
        if other.value == self.value {
            Ordering::Equal
        } else if other.value > self.value {
            Ordering::Greater
        } else {
            Ordering::Less
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
#[derive(Clone, Debug)]
pub struct Ranker {
    order : Vec<Rank>,
}

/// Default Ranker functions
impl Ranker {
    /// Makes a new Ranker
    pub fn new() -> Ranker {
        Ranker { order : Vec::new() }
    }

    /// Clears the ranking
    pub fn clear(&mut self) {
        self.order.clear();
    }

    /// Ranks all the users 
    pub fn rank_users(&mut self, user_manager : &UserManager, company_manager : &CompanyManager) -> Result<(), String> {
        //Reset the rankings
        self.clear();
        
        //Loop through every user
        for user in user_manager.users() {
            //Make a rank from each user
            match Rank::rank_from_user(user, company_manager) {
                Ok(new_rank) => self.order.push(new_rank),
                Err(error) => return Err(error),
            }
        }

        //Sorts the Users
        self.order.sort_by(|a, b| a.cmp(b));
        Ok(())
    }

    /// Gets the ranks in string to send over the server
    /// Gets the ranks from the range specified
    pub fn get_data_range(&self, mut range : std::ops::Range<usize>) -> Result<String, String> {
        // Keeps the range in the vectors bounds
        if range.start >= self.order.len() { return Ok(String::new()); /*return Err(format!("Start of range is out of bounds {}", range.start));*/ }
        if range.end > self.order.len() { range.end = self.order.len(); /*return Err(format!("End of range is out of bounds {}", range.end));*/ }

        //Bounds the range to the size of the array
        let mut data = String::new();

        //Loops through each rank in the range
        for rank in self.order[range.clone()].iter().enumerate() {
            data.push_str(&(rank.0 + range.start + 1).to_string());
            data.push('_');
            data.push_str(&rank.1.get_data());
            data.push(',');
        };

        //Removes the extra data
        if data.len() != 0 { data.pop(); }

        Ok(data)
    }
}


/// Tracks all history of the rankers
pub struct RankerHistory {
    history : Vec<Ranker>,
}


impl RankerHistory {
    /// Makes a new Ranker
    pub fn new() -> RankerHistory {
        RankerHistory { history: Vec::new() }
    }

    /// Adds a ranker to the list
    pub fn add(&mut self, ranker : Ranker) {
        self.history.push(ranker);
    }

    /// Gets the last ranking in the history
    pub fn get_recent(&self) -> Option<&Ranker> {
        self.history.last()
    }
}