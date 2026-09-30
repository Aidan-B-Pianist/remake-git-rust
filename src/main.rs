use remake_git_rust::Repository;

fn main() {
    println!("Testing Git Functions From Scratch (NO AI)\n");
    
    // Dummy test case
    // Switching from Vec<Object> to Hash<i32, i64> 
    let mut local = Repository::new();
    let mut server = Repository::new();

    println!("Printing our empty local: {:?}", local);
    println!("Printing our empty server: {:?}", server);
    
}

