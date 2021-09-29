use std::io;

fn main() {
    const CHARGES: f32 = 0.50;

    let mut withdraw = String::new();

    io::stdin()
        .read_line(&mut withdraw)
        .expect("Failed to read line");

    let withdraw: f32 = withdraw.trim().parse().expect("Input not a float");

    let mut balance = String::new();

    io::stdin()
        .read_line(&mut balance)
        .expect("Failed to read line");

    let mut balance: f32 = balance.trim().parse().expect("Input not a float");

    if (withdraw%5.0 == 0.0) && (withdraw<balance) && (withdraw>0.0) {
        balance = balance-withdraw-CHARGES;
    }

    println!("{}",balance);
}