use crate::storage::{load_json, save_json};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
    Deposit,
    Withdraw,
    TransferIn(String),  // TransferIn includes the account ID of the sender
    TransferOut(String), // TransferOut includes the account ID of the receiver
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub account_id: String,
    pub transaction_type: TransactionType,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: u64,
    pub address: String,
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accounts {
    pub account: Vec<Account>
}

impl Accounts {
    pub fn new() -> Self {
        Self {
            account: Vec::new(),
        }
    }

    pub fn load_from_file(path:&str)->Self{
        load_json(path).unwrap_or_else(|_| Accounts {
            account: Vec::new(),
        })
    }

    pub fn save_to_file(&mut self, path :&str) {
        if let Err(e) = save_json(path, self){
            eprintln!("Error saving account: {}", e);
        }
    }

    pub fn create_new_account(&mut self, name: &str, email: &str, phone: u64, address: &str) {
        if self.account.iter().any(|acc| acc.phone == phone) {
            println!("\nSorry! Account with this phone number already exists.");
            return;
        }

        let account = Account {
            id: phone.to_string(),
            name: name.to_owned(),
            email: email.to_owned(),
            phone,
            address: address.to_owned(),
            balance: 0.0,
            transactions: Vec::new(),
        };
        self.account.push(account);
        self.save_to_file("./jsons/accounts.json");
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

    pub fn deposit(&mut self, account_id: &str, amount: f64) {
        if amount <= 0.0 {
            println!("⚠️ Invalid deposit amount. Please enter a positive number.");
            return;
        }
        if !self.check_account_exists(account_id) {
            println!("⚠️ Account with ID {} not found.", account_id);
            return;
        }
        if let Some(account) = self.account.iter_mut().find(|acc| acc.id == account_id) {
            account.balance += amount as f64;
            self.create_transaction(account_id, TransactionType::Deposit, amount);
            println!("Deposited ${} to account ID : {}", amount, account_id);
        } else {
            println!("Account with ID {} not found.", account_id);
        }
    }

    pub fn withdraw(&mut self, account_id: &str, amount: f64) {
        if amount <= 0.0 {
            println!("⚠️ Invalid withdrawal amount. Please enter a positive number.");
            return;
        }
        if !self.check_account_exists(account_id) {
            println!("⚠️ Account with ID {} not found.", account_id);
            return;
        }
        if let Some(account) = self.account.iter_mut().find(|acc| acc.id == account_id) {
            if account.balance >= amount as f64 {
                account.balance -= amount as f64;
                self.create_transaction(account_id, TransactionType::Withdraw, amount);
                println!("Withdrew ${} from account ID : {}", amount, account_id);
            } else {
                println!("Insufficient balance in account ID : {}", account_id);
            }
        } else {
            println!("Account with ID {} not found.", account_id);
        }
    }

    pub fn show_transactions(&self, account_id: &str) {
        if let Some(acc) = self.account.iter().find(|acc| acc.id == account_id) {
            if acc.transactions.is_empty() {
                println!("No transactions found for account ID: {}", account_id);
            } else {
                print!("\nTransactions for account ID {}:\n", account_id);
                for transaction in &acc.transactions {
                    match transaction.transaction_type {
                        TransactionType::Deposit => {
                            println!("Deposited: ${:.2}", transaction.amount);
                        }
                        TransactionType::Withdraw => {
                            println!("Withdrew: ${:.2}", transaction.amount);
                        }
                        TransactionType::TransferIn(ref from_account_id) => {
                            println!(
                                "Transferred: ${:.2} from account ID {}",
                                transaction.amount, from_account_id
                            );
                        }
                        TransactionType::TransferOut(ref to_account_id) => {
                            println!(
                                "Transferred: ${:.2} to account ID {}",
                                transaction.amount, to_account_id
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn show_balance(&self, account_id: &str) {
        let accounts:Accounts = load_json("./jsons/accounts.json").expect("Accounts not found");
        if let Some(account) = accounts.account.iter().find(|acc| acc.id == account_id) {
            println!(
                "Balance for account ID {}: ${:.2}",
                account.id, account.balance
            );
        } else {
            println!("Account with ID {} not found.", account_id);
        }
    }

    pub fn check_account_exists(&self, account_id: &str) -> bool {
        self.account.iter().any(|acc| acc.id == account_id)
    }

    pub fn create_transaction(
        &mut self,
        account_id: &str,
        transaction_type: TransactionType,
        amount: f64,
    ) {
        if let Some(account) = self.account.iter_mut().find(|acc| acc.id == account_id) {
            let transaction = Transaction {
                account_id: account.id.clone(),
                transaction_type,
                amount,
            };
            println!(
                "Transaction of type {:?} for ${} created for account ID: {}",
                transaction.transaction_type, transaction.amount, transaction.account_id
            );

            account.transactions.push(transaction);
            let transactions = &account.transactions;
            save_json("./jsons/transactions.json", &transactions).unwrap();
        }
    }

    pub fn transfer_funds(&mut self, from_account_id: &str, to_account_id: &str, amount: f64) {
        if from_account_id == to_account_id {
            println!("⚠️ Cannot transfer to the same account.");
            return;
        }
        if !self.check_account_exists(from_account_id) {
            println!("⚠️ From account ID {} does not exist.", from_account_id);
            return;
        }
        if !self.check_account_exists(to_account_id) {
            println!("⚠️ To account ID {} does not exist.", to_account_id);
            return;
        }
        if let Some(from_account) = self.account.iter_mut().find(|acc| acc.id == from_account_id) {
            if from_account.balance < amount {
                println!(
                    "⚠️ Insufficient balance in account ID {} for transfer.",
                    from_account_id
                );
                return;
            }
            let from_index = self
                .account
                .iter()
                .position(|acc| acc.id == from_account_id)
                .unwrap();
            let to_index = self
                .account
                .iter()
                .position(|acc| acc.id == to_account_id)
                .unwrap();

            let (from_account, to_account) = if from_index > to_index {
                let (left, right) = self.account.split_at_mut(from_index);
                (&mut left[from_index], &mut right[0])
            } else {
                let (left, right) = self.account.split_at_mut(to_index);
                (&mut left[from_index], &mut right[0])
            };
            from_account.balance -= amount;
            to_account.balance += amount;
            self.create_transaction(
                from_account_id,
                TransactionType::TransferOut(to_account_id.to_string()),
                amount,
            );
            self.create_transaction(
                to_account_id,
                TransactionType::TransferIn(from_account_id.to_string()),
                amount,
            );
            println!(
                "Transferred ${} from account ID {} to account ID {}",
                amount, from_account_id, to_account_id
            );
        }
    }
}
