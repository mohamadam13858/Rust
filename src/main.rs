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

fn is_verified(account: &BankAcount) -> Result<bool, bool> {
    return match account.verified {
        true => Ok(true),
        false => Err(false),
    };
}

fn main() {
    let mohamad_account = BankAcount {
        balance: 10000,
        verified: false,
    };

    let verification_status = is_verified(&mohamad_account).expect("unable to unwrap result");

    print_balance(&mohamad_account);
    print_verified(&mohamad_account);

    println!("{:?}", verification_status)
}
