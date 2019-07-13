//! # A state-spill-free server-client generic (ssfscg)
//! 
//! ## 1. Design Goal
//! 1. state-spill free: no important states stored in server, and could be recovered when server lost them (hot upgrade)
//! 2. safe rust only: do not use `unsafe` code anywhere
//! 3. high robustness server: bad behaved clients will not cause server locking anyway
//! 4. high performance: \[optional\] reduce mutex competition by droppable checkpoint stored in server
//! 
//! ## 2. Usage Guide
//! You can run `cargo test -- --nocapture` first before actually reading how to use it
//! The test case includes detailed explanation of what server and clients do and its effect
//! 
//! ### Server
//! 

// for test, std crates are used, and for no_std environment the alternative crates should be provided
use std::sync::{Mutex, Weak, Arc};
use std::vec::{Vec};
use std::option::{Option};
#[macro_use] extern crate lazy_static;

/// structure that should be owned by server thread, record droppable states of clients
#[derive(Debug)]
pub struct Server<T> {
    pub clients: Vec<ServerSideClientObj<T>>,
}

impl<T> Server<T> {
    /// new Server object with initial status
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
        }
    }

    /// check whether a client has been in the clients queue
    pub fn has_client(&self, obj: &Arc<Mutex<T>>) -> bool {
        /// there should be more efficient way for searching, like hash set
        for i in 0..self.clients.len() {
            if Some(m) = self.clients[i].upgrade() {
                println!("{:?}", m);
            }
        }
        false
    }
}

/// this is the object that server owns, referring to client's object and take checkpoint if feature enabled
#[derive(Debug)]
pub struct ServerSideClientObj<T> {
    pub pointer: Weak<Arc<Mutex<T>>>,
    pub checkpoint: Option<T>,  // this is used only when server checkpoint is enabled
}

// run test with "cargo test -- --nocapture"
#[cfg(test)]
mod tests {

    use std::sync::{Arc, Mutex, Once};
    use crate::Server;
    use std::thread;

    #[derive(Debug)]
    struct AppSharedObj {
        a: usize,
        b: usize,
    }

    lazy_static! {
        #[derive(Debug)]
        static ref SERVER: Server<AppSharedObj> = Server::<AppSharedObj>::new();
    }

    #[test]
    fn one_server_one_client() {
        let server_thread = thread::spawn(|| {
            println!("1. initial status: {:?}", SERVER);
        });
        let client_thread_1 = thread::spawn(|| {
            let mut client = Arc::new(Mutex::new(AppSharedObj{
                a: 0,
                b: 0,
            }));
            println!("client in server: {}", SERVER.has_client(&client));
        });
        server_thread.join();
    }
}
