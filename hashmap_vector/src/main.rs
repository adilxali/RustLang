use std::collections::HashMap;

fn main() {
    let mut guest_hobbies: HashMap<String , Vec<String>> = HashMap::new();
    guest_hobbies.insert("Adil".to_string(), vec!["Reading".into(), "Coding".into()]);
    guest_hobbies.insert("Ali".to_string(), vec!["Gaming".into(), "Traveling".into()]);

    for (guest, hobbies) in &guest_hobbies {
        println!("{} enjoys:", guest);
        for hobby in hobbies {
            println!(" - {}", hobby);
        }
    }
}
