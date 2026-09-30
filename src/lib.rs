use std::{collections::HashMap, hash::Hash, vec};

// Every function will be in the main file
// #[derive(Debug, Clone)]
// struct Object {
//     var: i64,
//     key: i32,
// }
// //Vec --> HashMap 
//Add a real commit with a parent pointer moving from free functions to a repository struct
// Repository needs a staged, commits, and head, having methods such as repo.add and repo.commit("msg")

#[derive(Debug)]
struct Commit {
    snapshot: HashMap<i32, i64>,
    message: String,
    parent: Option<usize>
}

#[derive(Debug)]
pub struct Repository {
    staged: HashMap<i32, i64>,
    commits: Vec<Commit>,
    head: Option<usize>
}

impl Repository {
    pub fn new() -> Repository {
        Repository { staged: HashMap::new(), commits: vec![], head: None }
    }

    pub fn add() {

    }

    pub fn commit() {
    
    }
}
// mod Branch {
 

// use crate::{Object, Repository};

//     // add a file locally to the branch
//     pub fn add(obj: Object) {
//         local.push(obj);
//     }
    
//     // pull anything from a server given a key
//     pub fn pull(&mut key: i32) {
       
//         // pull all objects from server to home state
         
//     }

//     // push any changes from a branch to a server
//     pub fn push(server: &mut Vec<Object>, local: &mut Vec<Object>) {
//         // Get payload state and push it to server
        
//         // push all objects from local to server
//         // get key and if it's in server already, overwrite it
//          for item in local {
//             let local_key = item.key;
//             if(server.iter().any(|x| x.key == local_key)) {
//                 server.pop();
 
//             }
//             server.push(item.clone());
//         }
//     }

//     // write a commit message
//     pub fn commit() -> String {
//         // Save all final changes to the local state
//         String::from("pushed some new features")
//     }    
//