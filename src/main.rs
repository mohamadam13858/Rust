struct BankAcount {
    balance: i32,
    verified: bool,
}

fn main() {
    let mohamad_account = BankAcount {
        balance: 10000,
        verified: true,
    };
    let ali = BankAcount {
        balance: 20000000,
        verified: false,
    };

    println!("{:?}", ali.balance)
}
