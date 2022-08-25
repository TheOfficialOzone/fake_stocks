

use crate::User;
use crate::users::password::Password;
use crate::SaveData;
use crate::ID;

/// User Manager stores all the users in a Vectorp
pub struct UserManager {
    users : Vec<User>,
}

/// Default User Manager functions
impl UserManager {
    /// Creates a new User manager
    pub fn new() -> UserManager {
        UserManager {
            users: Vec::new(),
        }
    }

    /// Makes a new User
    pub fn new_user(&mut self, user_name : String, display_name : String, password : Password) -> Result<ID, String> {
        // Verifies that no user with the same user name or display name exist
        if let Ok(_user) = self.get_user_by_username(&user_name) { return Err(format!("User with user name {} already exists!", user_name)); }
        if let Ok(_user) = self.get_user_by_display_name(&display_name) { return Err(format!("User with display name {} already exists!", display_name)); }
        
        //Generates the new user
        let new_user = User::new(user_name, display_name, password);
        //Copies the ID for return
        let user_id = new_user.id();
        self.users.push(new_user);

        Ok(user_id)
    }

    /// Resets all the users
    pub fn reset_users(&mut self) {
        //Loops through all the users
        for user in self.users_mut() {
            user.reset();
        }
    }

    /// Getters

    /// Gets the users from the User manager
    pub fn users(&self) -> &Vec<User> {
        &self.users
    }

    /// Gets the users mutably from the User manager
    pub fn users_mut(&mut self) -> &mut Vec<User> {
        &mut self.users
    }

    /// Gets a user by their User name
    pub fn get_user_by_username(&self, username : &String) -> Result<&User, String> {
        let users = self.users();

        //Filters for identical user names
        let filtered : Vec<&User> = users
            .iter()
            .filter(|user| user.user_name().eq(username))
            .collect();

        //Checks that the lengths are correct
        if filtered.len() == 0 { return Err(format!("No User with name {} found", username)); }
        if filtered.len() != 1 { return Err(format!("Multiple Users with Name {} found", username)); }

        //Returns the remaining user
        Ok(filtered[0])
    }

    /// Gets a user by their Display name
    pub fn get_user_by_display_name(&self, display_name : &String) -> Result<&User, String> {
        let users = self.users();

        //Filters for identical user names
        let filtered : Vec<&User> = users
            .iter()
            .filter(|user| user.display_name().eq(display_name))
            .collect();
        
        //Checks that the lengths are correct
        if filtered.len() == 0 { return Err(format!("No User with name {} found", display_name)); }
        if filtered.len() != 1 { return Err(format!("Multiple Users with Name {} found", display_name)); }

        //Returns the remaining user
        Ok(filtered[0])
    }

    /// Gets a user by their ID
    pub fn get_user_by_id(&self, id : ID) -> Result<&User, String> {
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

    /// Gets the users by it's ID mutably
    pub fn get_user_by_id_mut(&mut self, id : ID) -> Result<&mut User, String> {
        //Loops through every user until it find a user with a matching id
        for user in self.users_mut() {
            if user.id().equals(id) {
                return Ok(user);
            }
        }

        Err(format!("No User with id {} found", id.value()))
    }
}

/// Allows the User Manager to save Data
impl SaveData for UserManager {
    /// Gets the data of the User Manager
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