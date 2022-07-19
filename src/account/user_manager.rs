

use crate::User;
use crate::SaveData;
use crate::id::ID;

pub struct UserManager {
    users : Vec<User>,
}

impl UserManager {
    //Creates a new User manager
    pub fn new() -> UserManager {
        UserManager {
            users: Vec::new(),
        }
    }

    //Makes a new User
    pub fn new_user(&mut self, name : String, money : f32) {
        //Generates the new user
        let new_user = User::new(name, money);
        self.users.push(new_user);
    }

    /// Getters

    //Gets the users from the User manager
    pub fn users(&self) -> &Vec<User> {
        &self.users
    }

    //Gets the user at the specified Position
    pub fn get_user(&self, pos : usize) -> &User {
        &self.users[pos]
    }

    //Gets the user at the specified Position mutably
    pub fn get_user_mut(&mut self, pos : usize) -> &mut User {
        &mut self.users[pos]
    }

    //Gets a user by their ID
    pub fn get_user_by_id(&self, id : &ID) -> Result<&User, String> {
        let users = self.users();

        //Filters the Users
        let filtered : Vec<&User> = users
            .iter()
            .filter(|user| user.id().value() == id.value())
            .collect();
        
        //Checks that the lengths are correct
        if filtered.len() == 0 { return Err(format!("No User with id {} found", id.value())); }
        if filtered.len() != 1 { return Err(format!("Multiple Users with ID {} found", id.value())); }

        Ok(filtered[0])
    }
}


/*
Allows the user to Save Data
 */
impl SaveData for UserManager {
    fn get_data(&self) -> String {
        //Starts with the name of the company
        let mut data : String = String::new();

        //Loop through each user adding their data
        for user in self.users() {
            data.push_str(&user.get_data());
            data.push('\n');
        }

        //Return the data
        data
    }
}