pub struct Account {
    id: String,
    name: String,
    email: String,
    phone: u32,
    address: String,
    balance: f64,
}

pub struct Accounts {
    account: Vec<Account>,
}

impl Accounts {
    pub fn new () -> Self {
       Self {
           account: Vec::new(),
        }
    }

    pub fn create_new_account(&mut self, name:&str, email:&str, phone:u32, address:&str) {
        if self.account.iter().any(|acc| acc.phone == phone) {
            println!("\nSorry! Account with this phone number already exists.");
            return;
        }
        let id = format!("{}-{}", name, phone);
        let account = Account {
            id,
            name: name.to_owned(),
            email: email.to_owned(),
            phone,
            address: address.to_owned(),
            balance: 0.0,
        };
        self.account.push(account);
        println!("Account created successfully with ID: {}", self.account.last().unwrap().id);
    }

    pub fn show_account_details(&self){
        if self.account.is_empty() {
            println!("No accounts available.");
            return;
        }else {

            println!("\nAccount Details:");
            println!("-------------------");
            for account in self.account.iter() {
                println!("ID: {}", account.id);
                println!("Name: {}", account.name);
                println!("Email: {}", account.email);
                println!("Phone: {}", account.phone);
                println!("Address: {}", account.address);
                println!("Balance: ${:.2}\n", account.balance);
            }
        }
    }
}