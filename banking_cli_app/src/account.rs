#[derive(Debug)]

pub enum TransactionType {
    Deposit,
    Withdraw,
}
#[derive(Debug)]
pub struct Transaction {
    pub account_id: String,
    pub transaction_type: TransactionType,
    pub amount: f64,
}

#[derive(Debug)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: u64,
    pub address: String,
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug)]
pub struct Accounts {
    pub account: Vec<Account>,
}

impl Accounts {
    pub fn new() -> Self {
        Self {
            account: Vec::new(),
        }
    }

    pub fn create_new_account(&mut self, name: &str, email: &str, phone: u64, address: &str) {
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
            transactions: Vec::new(),
        };
        self.account.push(account);
        println!(
            "Account created successfully with ID: {}",
            self.account.last().unwrap().id
        );
    }

    pub fn show_account_details(&self) {
        if self.account.is_empty() {
            println!("No accounts available.");
            return;
        } else {
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

    pub fn deposit(&mut self, account_id: &str , amount : f64){
        if let Some(account) = self.account.iter_mut().find(|acc| acc.id == account_id){
            account.balance += amount as f64;
            let transaction = Transaction {
                account_id: account.id.clone(),
                transaction_type: TransactionType::Deposit,
                amount: amount as f64,
            };
            account.transactions.push(transaction);
            println!("Deposited ${} to account ID : {}", amount, account_id);   
        } else {
            println!("Account with ID {} not found.", account_id);
        }
    }

    pub fn withdraw(&mut self, account_id: &str, amount : f64){
        if let Some(account) = self.account.iter_mut().find(|acc| acc.id == account_id) {
            if account.balance >= amount as f64 {
                account.balance -= amount as f64;
                let transaction = Transaction {
                    account_id: account.id.clone(),
                    transaction_type: TransactionType::Withdraw,
                    amount: amount as f64,
                };
                account.transactions.push(transaction);
                println!("Withdrew ${} from account ID : {}", amount, account_id);
            } else {
                println!("Insufficient balance in account ID : {}", account_id);
            }
        } else {
            println!("Account with ID {} not found.", account_id);
        }
    }

    pub fn show_transactions(&self, account_id: &str){
        if let Some(acc) = self.account.iter().find(|acc|acc.id == account_id){
            if acc.transactions.is_empty(){
                println!("No transactions found for account ID: {}", account_id);
            }else {
                print!("\nTransactions for account ID {}:\n", account_id);
                for transaction in &acc.transactions {
                    match transaction.transaction_type {
                        TransactionType::Deposit => {
                            println!("Deposited: ${:.2}", transaction.amount);
                        }
                        TransactionType::Withdraw => {
                            println!("Withdrew: ${:.2}", transaction.amount);
                        }
                    }
                }
            }
        }
    }

    pub fn show_balance(&self, account_id: &str) {
        if let Some(account) = self.account.iter().find(|acc| acc.id == account_id) {
            println!("Balance for account ID {}: ${:.2}", account.id, account.balance);
        } else {
            println!("Account with ID {} not found.", account_id);
        }
    }

}
