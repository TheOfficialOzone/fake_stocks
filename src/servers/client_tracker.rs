

use crate::id::ID;


/// Stores a client by their IP and user ID
struct ConnectedClient {
    id : ID,
    user_id : ID,
    user_name : String,
}


impl ConnectedClient {
    /// Creates a new Connected client from the IP and user_id
    fn new(user_id : ID, user_name : String) -> ConnectedClient {
        ConnectedClient { id : ID::new(), user_id, user_name }
    }

    /// Checks if two clients are identical
    fn equal(&self, other : &ConnectedClient) -> bool {
        self.id.equals(other.id)
    }

    /// Checks if the IDs are identical
    fn equal_id(&self, other_id : ID) -> bool {
        self.id.equals(other_id)
    }

    /// Checks if the two user IDs are identical
    fn equal_user_id(&self, other_user_id : ID) -> bool {
        self.user_id.equals(other_user_id)
    }

    /// Gets the user id
    fn user_id(&self) -> ID {
        self.user_id
    }

    /// Gets the clients ID
    fn client_id(&self) -> ID {
        self.id
    }

    /// Gets the clients User name
    fn user_name(&self) -> &String {
        &self.user_name
    }
}

/// Tracks the IPs of the user that are connected
pub struct ClientTracker {
    clients : Vec<ConnectedClient>,
}


impl ClientTracker {
    /// Creates a new empty Socket tracker
    pub fn new() -> ClientTracker {
        ClientTracker { clients: Vec::new() }
    }

    /// Adds a client to the list
    /// Returns the new clients ID
    pub fn add_client(&mut self, user_id : ID, user_name : String) -> Result<ID, String> {
        if self.contains_user_id(user_id) { return Err(String::from("Already has a client with that user ID Address")); }

        let new_client = ConnectedClient::new(user_id, user_name);
        let stored_id = new_client.client_id();
        self.clients.push(new_client);

        Ok(stored_id)
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
        Ok(filtered[0].user_id())
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
        Ok(filtered[0].client_id())
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

        // The length should be 1
        filtered.len() == 1
    }

    /// Checks if the client tracker contains the user name
    pub fn contains_user_name(&self, user_name : &String) -> bool {
        // Filters all clients for matching addresses
        let filtered : Vec<&ConnectedClient> = self.clients
            .iter()
            .filter(|client| client.user_name.eq(user_name))
            .collect();

        // The length should be 1
        filtered.len() == 1
    }
}