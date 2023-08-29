use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::num::ParseIntError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentArgs {
    pub amount: f64,
    pub memo: String,
    pub metadata: Value,
    pub uid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub amount: f64,
    pub payment_identifier: String,
    pub from_address: String,
    pub to_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDTO {
    pub identifier: String,
    pub user_uid: String,
    pub amount: f64,
    pub memo: String,
    pub metadata: Value,
    pub from_address: String,
    pub to_address: String,
    pub direction: Direction,
    pub status: PaymentDTOStatus,
    pub transaction: Option<PaymentDTOTransaction>,
    pub created_at: String,
    pub network: NetworkPassphrase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDTOStatus {
    pub developer_approved: bool,
    pub transaction_verified: bool,
    pub developer_completed: bool,
    pub cancelled: bool,
    pub user_cancelled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDTOTransaction {
    pub txid: String,
    pub verified: bool,
    pub _link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqwestClientOptions {
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "user_to_app")]
    UserToApp,
    #[serde(rename = "app_to_user")]
    AppToUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkPassphrase {
    #[serde(rename = "Pi Network")]
    PiNetwork,
    #[serde(rename = "Pi Testnet")]
    PiTestnet,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IncompletePaymentResponse {
    pub incomplete_server_payments: Vec<PaymentDTO>,
}

#[derive(Debug)]
pub enum PiError {
    Message(String),
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
    Anyhow(anyhow::Error),
    ParseError(ParseIntError),
}

impl std::fmt::Display for PiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PiError::Message(ref msg) => write!(f, "{}", msg),
            PiError::Reqwest(ref err) => write!(f, "Reqwest error: {}", err),
            PiError::Json(ref err) => write!(f, "JSON error: {}", err),
            PiError::Anyhow(ref err) => write!(f, "Horizon error: {}", err),
            PiError::ParseError(ref err) => write!(f, "Can't parse: {}", err),
        }
    }
}

impl std::error::Error for PiError {}

impl From<reqwest::Error> for PiError {
    fn from(err: reqwest::Error) -> Self {
        PiError::Reqwest(err)
    }
}

impl From<serde_json::Error> for PiError {
    fn from(err: serde_json::Error) -> Self {
        PiError::Json(err)
    }
}

impl From<anyhow::Error> for PiError {
    fn from(err: anyhow::Error) -> Self {
        PiError::Anyhow(err)
    }
}

impl From<ParseIntError> for PiError {
    fn from(err: ParseIntError) -> Self {
        PiError::ParseError(err)
    }
}
