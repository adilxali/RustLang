mod utils;

// Immutable Brorrowing (&T)

fn print_name(name:&String){
    println!("Name is : {}", name)
}

// Mutable Borrowing ( &mut T)
fn make_upper(name:&mut String){
    name.push_str(" Ali");
}

fn display_username(username: &String){
    println!("Username is : {}", username)
}


fn main() {
    println!("Hello, world!");
    let full_name = utils::create_full_name("Adil", "Ali");
    println!("My fullname is {}", full_name);
    utils::if_elseif_else(90);
    utils::_match(400);
    utils::_loop();
    utils::_for();
    utils::_ownership();
    print_name(&full_name);
    // print_name(full_name); it will throw error fn expect Slice String not string

    let mut user: String = String::from("Adil");
    make_upper(&mut user);
    // let user = "Adil";
    // make_upper(&mut user); it will throw error it expecting a mut variable

    let mut name: String = String::from("Adil");

    // Step 1: Call display_username (borrowed)
    display_username(&name);

    // Step 2: Mutably borrow and append " Ali"
    name.push_str(" Ali");

    // Step 3: Print final name
    println!("Full name: {}", name);
}
