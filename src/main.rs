use rust_transaction_engine::run;
use std::process;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}