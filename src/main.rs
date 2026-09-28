// Every function will be in the main file
#[derive(Debug, Clone)]
struct Object {
    var: i64,
    key: i32,
}


fn main() {
    println!("Testing Git Functions From Scratch (NO AI)\n");
    
    // Dummy test case
    let mut obj0 = Object{var: 5324987, key: 0};
    let mut local:Vec<Object> = vec![];
    let mut server:Vec<Object> = vec![];
    let mut obj1: Object = Object { var: 234783, key: 1,};
    server.push(obj0);
    
    println!("----- Starting State -----");
    println!("Object with key 1 is: {:?}", obj1); 
    println!("Server state has: {:?}", server);
    println!("Local state has: {:?}\n", local);

    //changing obj to a new var w/ same key
    obj1.var = 23;

    Branch::add(&mut local, obj1);
    println!("----- After Adding Files to Payload (local) -----");
    println!("Object with key 1 is: {:?}", local[0]); 
    println!("Server state has: {:?}", server);
    println!("Local state has: {:?}\n", local);

    let string = Branch::commit();
    println!("----- After Commit -----");
    println!("Object with key 1 is: {:?}", string); 
    println!("Server state has: {:?}", server);
    println!("Local state has: {:?}\n", local);

    Branch::push(&mut server, &mut local);
    println!("----- After Pushing W/ Update -----"); 
    println!("Server state has: {:?}", server);
    println!("Local state has: {:?}\n", local);

    // Branch::pull(server, 1);
    Branch::pull(&mut server, &mut local);
    println!("----- After Pulling -----");
    println!("Server state has: {:?}", server);
    println!("Local state has: {:?}\n", local);
}

mod Branch {
    use crate::Object;

    // add a file locally to the branch
    pub fn add(local: &mut Vec<Object>, obj: Object) {
        local.push(obj);
    }
    
    // pull anything from a server given a key
    pub fn pull(server: &mut Vec<Object>, local: &mut Vec<Object>) {
       
        // pull all objects from server to home state

        for item in server {
            let local_key = item.key;
            if(local.iter().any(|x| x.key == local_key)) {
                local.pop();
            }
            local.push(item.clone());
        }

    }

    // push any changes from a branch to a server
    pub fn push(server: &mut Vec<Object>, local: &mut Vec<Object>) {
        // Get payload state and push it to server
        
        // push all objects from local to server
        // get key and if it's in server already, overwrite it
         for item in local {
            let local_key = item.key;
            if(server.iter().any(|x| x.key == local_key)) {
                server.pop();
 
            }
            server.push(item.clone());
        }
    }

    // write a commit message
    pub fn commit() -> String {
        // Save all final changes to the local state
        String::from("pushed some new features")
    }    
}
