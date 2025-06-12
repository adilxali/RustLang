use std::io;

// Mutable Borrow
fn add_guest(guest_list: &mut Vec<String>, name:String){
    guest_list.push(name);
}

// immutable borrow

fn print_guests(guest_list:&Vec<String>){
    println!("Guest List:");
    for guest in guest_list {
        println!("- {}", guest);
    }
}

fn main() {
    let mut guests:Vec<String> = Vec::new();
    /* 
   with Statically Assigned Value
   
   

    add_guest(&mut guests, "Adil".to_string());
    add_guest(&mut guests, "Ali".to_string());

    print_guests(&guests);
    
    */
    
    // Terminal Input Output
    loop {
        println!("Please enter your name (or 'exit') : ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        
        let name = name.trim();
        if name == "exit"{
            break;
        }
        add_guest(&mut guests, name.to_string());
    }
    print_guests(&guests);
}
