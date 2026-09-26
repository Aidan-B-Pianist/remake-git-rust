// Every function will be in the main file
#[derive(Debug, Clone)]
struct Object {
    var: i64,
    key: i32,
}


fn main() {
    println!("Testing Git Functions From Scratch (NO AI)");
    
    // Dummy test case
    let mut local:Vec<Object> = vec![];
    let mut server:Vec<Object> = vec![Object{var: 5324987, key: 0}];
    let mut obj: Object = Object { var: 234783, key: 1 };
    
    println!("Object with key 1 is: {:?}", obj); 
    println!("Server state has: {:?}", server);
    println!("Local state has: {:?}", local);

    //changing obj to a new var w/ same key
    obj.var = 23;

    Branch::commit(local, obj);
    println!("----- After Commit -----");
    println!("Object with key 1 is: {:?}", obj); 
    println!("Server state has: {:?}", server);
    println!("Local state has: {:?}", local);

    // Branch::push(server, obj.clone());
    println!("----- After Pushing W/ Update -----");
    println!("Object with key 1 is: {:?}", obj); 
    println!("Server state has: {:?}", server);
    println!("Local state has: {:?}", local);

    // Branch::pull(server, 1);
    println!("----- After Pulling -----");
    println!("Object with key 1 is: {:?}", obj); 
    println!("Server state has: {:?}", server);
    println!("Local state has: {:?}", local);
}

mod Branch {
    use crate::Object;

    pub fn pull(mut server: Vec<Object>, key_value: i32) -> Option<Object> {
        
        if let Some(obj:Object) = server.into_iter().find(|x| x.key == key_value).or(None);
    }

    pub fn push(mut server: Vec<Object>, obj: Object) {
        // Assume we want to push to the server state
        server.push(obj);
    }

    pub fn commit(mut local: Vec<Object>, obj: Object) {
        // Save all final changes to the local state
        local.push(obj);
    }    
}
