struct BankAcount {
    balance: i32,
    verified: bool,
}

fn print_balance(account: &BankAcount) {
    println!("{:?}", account.balance)
}
fn print_verified(account: &BankAcount) {
    println!("{:?}", account.verified)
}

fn main() {
    let mohamad_account = BankAcount {
        balance: 10000,
        verified: true,
    };

    print_balance(&mohamad_account);
    print_verified(&mohamad_account);
}
