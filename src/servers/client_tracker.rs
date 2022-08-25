

use crate::id::ID;


/// Stores a client by their IP and user ID
#[derive(Clone)]
pub struct ConnectedClient {
    client_id : ID,
    user_id : ID,
    user_name : String,
    display_name : String,
}


impl ConnectedClient {
    /// Creates a new Connected client from the IP and user_id
    fn new(user_id : ID, user_name : String, display_name : String) -> ConnectedClient {
        ConnectedClient { client_id : ID::new(), user_id, user_name, display_name }
    }

    /// Gets the client ID
    pub fn client_id(&self) -> ID {
        self.client_id
    }

    /// Gets the user ID
    pub fn user_id(&self) -> ID {
        self.user_id
    }

    /// Gets the username
    pub fn user_name(&self) -> &String {
        &self.user_name
    }

    /// Checks if the IDs are identical
    fn equal_id(&self, other_id : ID) -> bool {
        self.client_id.equals(other_id)
    }

    /// Checks if the two user IDs are identical
    fn equal_user_id(&self, other_user_id : ID) -> bool {
        self.user_id.equals(other_user_id)
    }
}

/// Tracks the IPs of the user that are connected
#[derive(Clone)]
pub struct ClientTracker {
    clients : Vec<ConnectedClient>,
}

impl ClientTracker {
    /// Creates a new empty Socket tracker
    pub fn new() -> ClientTracker {
        ClientTracker { clients: Vec::new() }
    }

    /// Clears the client tracker
    pub fn clear(&mut self) {
        self.clients.clear()
    }

    /// Adds a client to the list
    /// Returns the new clients ID
    pub fn add_client(&mut self, user_id : ID, user_name : String, display_name : String) -> Result<ID, String> {
        //Ensures the client has not already been added
        if self.contains_user_id(user_id) { return Err(String::from("Already has a client with that user ID Address")); }
        if self.contains_user_name(&user_name) { return Err(format!("User name already in use: {}", user_name)); }
        if self.contains_display_name(&display_name) { return Err(format!("Display name already in use: {}", display_name)); }

        //Makes the new client
        let new_client = ConnectedClient::new(user_id, user_name, display_name);
        //Stores the ID to return
        let stored_id = new_client.client_id;
        self.clients.push(new_client);

        Ok(stored_id)
    }


    /// Gets the client by their ID
    pub fn get_client_by_client_id(&self, client_id : ID) -> Result<&ConnectedClient, String> {
        // Filters for the remaining client
        let filtered : Vec<&ConnectedClient> = self.clients.iter()
            .filter(|client| client.client_id().equals(client_id))
            .collect();

        match filtered.len() {
            0 => Err(format!("No client with ID {} exists.", client_id)),
            _ => Ok(filtered[0]),
        }
    }

    /// Gets a users ID from the clients ID
    pub fn get_user_id_by_client_id(&self, client_id : ID) -> Result<ID, String> {
        // Filters all clients for matching addresses
        let filtered : Vec<&ConnectedClient> = self.clients
            .iter()
            .filter(|client| client.equal_id(client_id))
            .collect();

        // The length should be 1
        if filtered.len() != 1 { return Err(format!("No client with ID {} found", client_id)); }

        // Gets the users ID
        Ok(filtered[0].user_id)
    }

    /// Gets a clients ID from the users ID
    pub fn get_client_id_by_user_id(&self, user_id : ID) -> Result<ID, String> {
        // Filters all clients for matching addresses
        let filtered : Vec<&ConnectedClient> = self.clients
            .iter()
            .filter(|client| client.equal_user_id(user_id))
            .collect();

        // The length should be 1
        if filtered.len() != 1 { return Err(format!("No client with ID {} found", user_id)); }

        // Gets the users ID
        Ok(filtered[0].client_id)
    }

    /// Checks if the client is already connected
    pub fn contains_client_id(&self, client_id : ID) -> bool {
        // Filters all clients for matching addresses
        let filtered : Vec<&ConnectedClient> = self.clients
            .iter()
            .filter(|client| client.equal_id(client_id))
            .collect();

        // The length should be 1
        filtered.len() == 1
    }

    /// Checks if the user is already contained
    pub fn contains_user_id(&self, user_id : ID) -> bool {
        // Filters all clients for matching addresses
        let filtered : Vec<&ConnectedClient> = self.clients
            .iter()
            .filter(|client| client.equal_user_id(user_id))
            .collect();

        // The length should not be 0
        filtered.len() != 0
    }

    /// Checks if the client tracker contains the user name
    pub fn contains_user_name(&self, user_name : &String) -> bool {
        // Filters all clients for matching addresses
        let filtered : Vec<&ConnectedClient> = self.clients
            .iter()
            .filter(|client| client.user_name.eq(user_name))
            .collect();

        // The length should not be 0
        filtered.len() != 0
    }

    /// Checks if the display name is already in the tracker
    pub fn contains_display_name(&self, display_name : &String) -> bool {
        // Filters all clients for matching addresses
        let filtered : Vec<&ConnectedClient> = self.clients
            .iter()
            .filter(|client| client.display_name.eq(display_name))
            .collect();

        // The length should be 1
        filtered.len() == 1
    }
}