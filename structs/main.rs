fn main() {
    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    account.check_balance();

    account.withdraw(50.5);

    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "WIthdrawing {} from account owned By {}",
            amount, self.owner
        );
        self.balance -= amount;
    }
    fn check_balance(&self) {
        println!("Account Balance:{}", self.balance);
    }
}
