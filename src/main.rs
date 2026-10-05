use std::{collections::HashMap, vec};

use remake_git_rust::{Commit, Repository};

fn main() {
    println!("Testing Git Functions From Scratch (NO AI)\n");
    
    // Dummy test case
    // Switching from Vec<Object> to Hash<i32, i64> 
    let mut local = Repository::new();
    let mut server = Repository::new();

    println!("Printing our empty local: {:?}", local);
    println!("Printing our empty server: {:?}", server);
    println!("----------------------------------------\n");

    let mut staged = HashMap::new();
    staged.insert(1, 56234);
    let mut commits = Vec::new();
    commits.push(Commit::new());
    local = Repository::add(staged, commits, None);
   
    println!("Printing our empty local: {:?}", local);
    println!("Printing our empty server: {:?}", server);
    
}

