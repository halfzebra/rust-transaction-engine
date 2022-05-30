use clap::Parser;
use rust_transaction_engine::account::Account;
use rust_transaction_engine::transaction::Transaction;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    input: PathBuf,
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let f = File::open(&args.input)?;
    let reader = BufReader::new(f);
    let mut rdr = csv::Reader::from_reader(reader);
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut history_per_client = BTreeMap::new();

    let mut accounts = BTreeMap::new();

    for result in rdr.deserialize() {
        let record: Transaction = result?;

        // Create an account for new client ids
        if !accounts.contains_key(&record.client) {
            accounts.insert(record.client, Account::new(record.client));
        }

        // Add a BTreeMap to store historical transactions
        if !history_per_client.contains_key(&record.client) {
            history_per_client.insert(record.client, BTreeMap::new());
        }

        let ts: &mut BTreeMap<u32, Transaction> =
            history_per_client.get_mut(&record.client).unwrap();
        let acc = accounts.get_mut(&record.client).unwrap();

        acc.apply_transaction(&record, ts.get_mut(&record.tx));
    }

    for (_client, account) in &accounts {
        wtr.serialize(&account)?;
    }

    wtr.flush()?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}