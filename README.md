# 🔄 rust-transaction-engine

## Usage

```bash
cargo run -- examples/basic.csv
# to output into accounts.csv file
cargo run -- examples/basic.csv > accounts.csv
```

## Possible optimisations

Streaming the CSV is a bit pointless, since we are doing stateful event stream processing.
This means that we actually need to access transactions retrospectively, so storing them in an indexed format in some persisted storage is a must.

As an option for storage we could use SQLite or other embedded KV solutions, in a real project it's probably better to use a DB.

## Assumptions

- It's only possible to `Dispute` the `Withdrawal` and `Deposit`
- It's impossible to `Withdraw` negative amount
- It's impossible to `Deposit` negative amount
- `Withdraw`al of larger sum, than available does not process and is ignored
- Locked account does not respond to any further transactions