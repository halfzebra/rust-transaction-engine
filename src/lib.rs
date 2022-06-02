use self::account::Account;
use self::transaction::Transaction;
use clap::Parser;
use csv::Trim;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::io;
use std::path::PathBuf;

pub mod account;
pub mod transaction;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    input: PathBuf,
}

fn process(p: &PathBuf) -> Result<BTreeMap<u16, Account>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .trim(Trim::All)
        .flexible(true)
        .from_path(&p)?;
    let mut history_per_client = HashMap::new();
    let mut accounts = BTreeMap::new();

    for result in rdr.deserialize() {
        let mut record: Transaction = result?;

        // Create an account for new client ids
        if !accounts.contains_key(&record.client) {
            accounts.insert(record.client, Account::new(record.client));
        }

        // Add a BTreeMap to store historical transactions
        if !history_per_client.contains_key(&record.client) {
            history_per_client.insert(record.client, HashMap::new());
        }

        let ts: &mut HashMap<u32, Transaction> =
            history_per_client.get_mut(&record.client).unwrap();

        let acc = accounts.get_mut(&record.client).unwrap();
        let prev = ts.get_mut(&record.tx);

        acc.apply_transaction(&mut record, prev);

        if !ts.contains_key(&record.tx) && record.can_be_disputed() {
            ts.insert(record.tx, record);
        }
    }

    Ok(accounts)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let accounts = process(&args.input)?;

    write_to_stdout(&accounts)?;

    Ok(())
}

fn write_to_stdout(accounts: &BTreeMap<u16, Account>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    for (_client, account) in accounts {
        wtr.serialize(&account)?;
    }

    wtr.flush()?;

    Ok(())
}
