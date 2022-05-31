use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum TStatus {
    Ok,
    Disputed,
    Chargedback,
}

impl Default for TStatus {
    fn default() -> Self {
        TStatus::Ok
    }
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(skip, default)]
    pub status: TStatus,

    #[serde(rename = "type")]
    pub tt: TType,
    pub client: u16,
    pub tx: u32,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub amount: Option<f64>,
}

impl Transaction {
    pub fn can_be_disputed(&self) -> bool {
        self.status == TStatus::Ok && (self.tt == TType::Withdrawal || self.tt == TType::Deposit)
    }

    pub fn can_complete_dispute(&self) -> bool {
        self.status == TStatus::Disputed
            && (self.tt == TType::Withdrawal || self.tt == TType::Deposit)
    }
}
