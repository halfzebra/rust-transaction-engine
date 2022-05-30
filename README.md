# rust-transaction-engine

## Usage

```bash
cargo run -- examples/basic.csv
# to output into accounts.csv file
cargo run -- examples/basic.csv > accounts.csv
```

## Assumptions

- It's only possible to `Dispute` the `Withdrawal` and `Deposit`
- It's impossible to `Withdraw` negative amount
- It's impossible to `Deposit` negative amount
- `Withdraw`al of larger sum, than available does not process
- Locked account does not respond to any further transactions