use super::transaction::{TStatus, TType, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Account {
    client: u16,
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}

impl Account {
    pub fn new(client: u16) -> Self {
        Account {
            client,
            available: 0.0000,
            held: 0.0000,
            total: 0.0000,
            locked: false,
        }
    }

    pub fn deposit(&mut self, amount: &f64) -> &mut Self {
        if !self.locked {
            self.available += amount;
            self.update_total();
        }
        self
    }

    pub fn withdraw(&mut self, amount: &f64) -> &mut Self {
        if !self.locked && self.available >= *amount {
            self.available -= amount;
            self.update_total();
        }
        self
    }

    pub fn resolve(&mut self, amount: &f64) -> &mut Self {
        if !self.locked && self.held >= *amount {
            self.available += amount;
            self.held -= amount;
        }
        self
    }

    pub fn lock(&mut self) -> &mut Self {
        if !self.locked {
            self.locked = true;
        }
        self
    }

    fn update_total(&mut self) -> &mut Self {
        self.total = self.available + self.held;
        self
    }

    pub fn apply_transaction(
        &mut self,
        transaction: &Transaction,
        preceeding_transaction: Option<&mut Transaction>,
    ) {
        match (&transaction.tt, preceeding_transaction) {
            (TType::Deposit, None) => {
                if let Some(amount) = &transaction.amount {
                    self.deposit(&amount);
                }
            }
            (TType::Withdrawal, None) => {
                if let Some(amount) = &transaction.amount {
                    self.withdraw(&amount);
                }
            }
            (TType::Dispute, Some(prev_transaction)) => {
                if !prev_transaction.can_be_disputed() {
                    return;
                }
                match (&prev_transaction.tt, &prev_transaction.amount) {
                    (TType::Deposit, Some(amount)) => {
                        prev_transaction.status = TStatus::Disputed;
                        self.held += amount;
                        self.available -= amount;
                        self.update_total();
                    }
                    (TType::Withdrawal, Some(amount)) => {
                        prev_transaction.status = TStatus::Disputed;
                        self.held += amount;
                        self.update_total();
                    }
                    _ => (),
                }
            }
            (TType::Resolve, Some(prev_transaction)) => {
                if !prev_transaction.can_complete_dispute() {
                    return;
                }
                match (&prev_transaction.tt, &prev_transaction.amount) {
                    (TType::Deposit, Some(amount)) => {
                        prev_transaction.status = TStatus::Ok;
                        self.held -= amount;
                        self.update_total();
                    }
                    (TType::Withdrawal, Some(amount)) => {
                        prev_transaction.status = TStatus::Ok;
                        self.held -= amount;
                        self.available += amount;
                        self.update_total();
                    }
                    _ => (),
                }
            }
            (TType::Chargeback, Some(prev_transaction)) => {
                if !prev_transaction.can_complete_dispute() {
                    return;
                }
                match (&prev_transaction.tt, &prev_transaction.amount) {
                    (TType::Deposit | TType::Withdrawal, Some(amount)) => {
                        prev_transaction.status = TStatus::Chargedback;
                        self.held -= amount;
                        self.update_total();
                        self.lock();
                    }
                    _ => (),
                }
            }
            _ => (),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_withdrawal() {
        let mut acc = Account::new(1u16);

        acc.deposit(&15.0);

        acc.apply_transaction(
            &Transaction {
                client: 1,
                tx: 1,
                tt: TType::Withdrawal,
                amount: Some(10.0),
                status: TStatus::Ok,
            },
            None,
        );

        assert_eq!(
            acc,
            Account {
                client: 1,
                total: 5.0,
                available: 5.0,
                held: 0.0,
                locked: false,
            }
        )
    }

    #[test]
    fn basic_deposit() {
        let mut acc = Account::new(1u16);

        let tr = Transaction {
            client: 1,
            tx: 1,
            tt: TType::Deposit,
            amount: Some(10.0),
            status: TStatus::Ok,
        };

        acc.apply_transaction(&tr, None);

        assert_eq!(
            acc,
            Account {
                client: 1,
                total: 10.0,
                available: 10.0,
                held: 0.0,
                locked: false,
            }
        );
    }

    #[test]
    fn withdrawal_dispute() {
        let mut acc = Account::new(1u16);

        acc.deposit(&15.0);

        let mut wtr = Transaction {
            client: 1,
            tx: 1,
            tt: TType::Withdrawal,
            amount: Some(10.0),
            status: TStatus::Ok,
        };
        
        acc.apply_transaction(&wtr, None);
        acc.apply_transaction(
            &Transaction {
                client: 1,
                tx: 1,
                tt: TType::Dispute,
                amount: None,
                status: TStatus::Ok,
            },
            Some(&mut wtr),
        );

        assert_eq!(acc.total, 15.0);
        assert_eq!(acc.available, 5.0);
        assert_eq!(acc.held, 10.0);
        assert_eq!(acc.locked, false);
        assert_eq!(wtr.status, TStatus::Disputed)
    }

    #[test]
    fn deposit_dispute() {
        let mut acc = Account::new(1u16);

        let mut dpstt = Transaction {
            client: 1,
            tx: 1,
            tt: TType::Deposit,
            amount: Some(10.0),
            status: TStatus::Ok,
        };

        let dispt = Transaction {
            client: 1,
            tx: 1,
            tt: TType::Dispute,
            amount: None,
            status: TStatus::Ok,
        };

        acc.apply_transaction(&dpstt, None);
        acc.apply_transaction(&dispt, Some(&mut dpstt));

        assert_eq!(acc.total, 10.0);
        assert_eq!(acc.available, 0.0);
        assert_eq!(acc.held, 10.0);
        assert_eq!(acc.locked, false);
        assert_eq!(dpstt.status, TStatus::Disputed)
    }

    #[test]
    fn withdrawal_dispute_resolve() {
        let mut acc = Account::new(1u16);

        acc.deposit(&15.0);

        let mut wtr = Transaction {
            client: 1,
            tx: 1,
            tt: TType::Withdrawal,
            amount: Some(10.0),
            status: TStatus::Ok,
        };
        acc.apply_transaction(&wtr, None);
        acc.apply_transaction(
            &Transaction {
                client: 1,
                tx: 1,
                tt: TType::Dispute,
                amount: None,
                status: TStatus::Ok,
            },
            Some(&mut wtr),
        );

        acc.apply_transaction(
            &Transaction {
                client: 1,
                tx: 1,
                tt: TType::Resolve,
                amount: None,
                status: TStatus::Ok,
            },
            Some(&mut wtr),
        );

        assert_eq!(acc.total, 15.0);
        assert_eq!(acc.available, 15.0);
        assert_eq!(acc.held, 0.0);
        assert_eq!(acc.locked, false);
        assert_eq!(wtr.status, TStatus::Ok)
    }

    #[test]
    fn deposit_dispute_resolve() {
        let mut acc = Account::new(1u16);

        let mut dpstt = Transaction {
            client: 1,
            tx: 1,
            tt: TType::Deposit,
            amount: Some(10.0),
            status: TStatus::Ok,
        };
        acc.apply_transaction(&dpstt, None);
        acc.apply_transaction(
            &Transaction {
                client: 1,
                tx: 1,
                tt: TType::Dispute,
                amount: None,
                status: TStatus::Ok,
            },
            Some(&mut dpstt),
        );
        acc.apply_transaction(
            &Transaction {
                client: 1,
                tx: 1,
                tt: TType::Resolve,
                amount: None,
                status: TStatus::Ok,
            },
            Some(&mut dpstt),
        );

        assert_eq!(acc.total, 0.0);
        assert_eq!(acc.available, 0.0);
        assert_eq!(acc.held, 0.0);
        assert_eq!(acc.locked, false);
        assert_eq!(dpstt.status, TStatus::Ok)
    }

    #[test]
    fn withdrawal_dispute_chargeback() {
        let mut acc = Account::new(1u16);

        acc.deposit(&15.0);

        let mut wtr = Transaction {
            client: 1,
            tx: 1,
            tt: TType::Withdrawal,
            amount: Some(10.0),
            status: TStatus::Ok,
        };
        acc.apply_transaction(&wtr, None);
        acc.apply_transaction(
            &Transaction {
                client: 1,
                tx: 1,
                tt: TType::Dispute,
                amount: None,
                status: TStatus::Ok,
            },
            Some(&mut wtr),
        );

        acc.apply_transaction(
            &Transaction {
                client: 1,
                tx: 1,
                tt: TType::Chargeback,
                amount: None,
                status: TStatus::Ok,
            },
            Some(&mut wtr),
        );

        dbg!(&acc);

        assert_eq!(acc.total, 5.0);
        assert_eq!(acc.available, 5.0);
        assert_eq!(acc.held, 0.0);
        assert_eq!(acc.locked, true);
        assert_eq!(wtr.status, TStatus::Chargedback)
    }

    #[test]
    fn deposit_dispute_chargeback() {
        let mut acc = Account::new(1u16);

        let mut dpstt = Transaction {
            client: 1,
            tx: 1,
            tt: TType::Deposit,
            amount: Some(10.0),
            status: TStatus::Ok,
        };
        acc.apply_transaction(&dpstt, None);
        acc.apply_transaction(
            &Transaction {
                client: 1,
                tx: 1,
                tt: TType::Dispute,
                amount: None,
                status: TStatus::Ok,
            },
            Some(&mut dpstt),
        );
        acc.apply_transaction(
            &Transaction {
                client: 1,
                tx: 1,
                tt: TType::Chargeback,
                amount: None,
                status: TStatus::Ok,
            },
            Some(&mut dpstt),
        );

        assert_eq!(acc.total, 0.0);
        assert_eq!(acc.available, 0.0);
        assert_eq!(acc.held, 0.0);
        assert_eq!(acc.locked, true);
        assert_eq!(dpstt.status, TStatus::Chargedback)
    }
}
