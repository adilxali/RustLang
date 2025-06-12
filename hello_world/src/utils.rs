pub fn create_full_name(firstname:&str, lastname:&str)->String{
    let full_name = format!("{} {}",firstname,lastname);
    full_name
}
pub fn if_elseif_else(score: u8) {
    if score >= 90 {
        println!("Excellent");
    } else if score >= 75 {
        println!("Good");
    } else {
        println!("Needs Improvement");
    }
}

pub fn _match(code:u32){
    match code {
        0 => println!("<UNK>"),
        200 => println!("Ok"),
        404 => println!("Not found"),
        500 => println!("Internal Server Error"),
        _ => println!("Unknown Status"),
    }
}

pub fn _loop(){
    let mut count = 0;
    //     loop method
    loop {
        if count == 3 {
            break;
        }
        println!("Count using (loop) {} \n", count);
        count+=1;
    }
    while count > 0 {
        print!("Count using (while) {} \n", count);
        count -=1;
    }
}

pub fn _for(){
    for i in 1..4{
        print!("Using for {} \n", i);
    }
}

pub fn _ownership(){
    let a = String::from("Hello");
    let b = a; // ownership moved from a to b if we try to print the error occure Error: a is no longer valid
    // println!("{}", a); Value being used after moved
    println!("{}", b); // this is right way for output

    let a = String::from("Adil");
    let b = a;         // Moves ownership — a is now invalid

    let c = String::from("Ali");
    let d = c.clone(); // ✅ Clones heap data — both c and d are valid

    let s = String::from("Rust");
    let r1 = &s;
    let r2 = &s;
    // both are OK
}