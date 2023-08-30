use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::num::ParseIntError;

/// Payment arguments:
///
/// - amount: f64 - the amount of Pi you're paying to your user
/// - memo: String - a short memo that describes what the payment is about, 28 english characters or 28-bytes
/// - metadata: Value - an arbitrary object that you can attach to this payment. This is for your own use. You should use this object as a way to link this payment with your internal business logic. serde_json::Value
/// - uid: String - a user uid of your app. You should have access to this value if a user has authenticated on your app.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentArgs {
    pub amount: f64,
    pub memo: String,
    pub metadata: Value,
    pub uid: String,
}

/// Transaction data:
///
/// Not necessary used by the crate's user it's an internal struct made by `submit_payment` function and consumed by `build_a2u_transaction`.
/// It carrying information necessary for the transaction building.
///     - amount: f64 - the amount of Pi you're paying to your user
///     - payment_identifier: String - This is generated by Pi API and this carried to the transaction building this will be the Memo's text
///     - from_address: String - From which address the transaction coming from
///     - to_address: String - To address where the transaction going to

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub amount: f64,
    pub payment_identifier: String,
    pub from_address: String,
    pub to_address: String,
}

/// Payment data generated by Pi API
///
/// - identifier: String - payment identifier
/// - user_uid: String - user's app-specific ID
/// - amount: f64 - payment amount
/// - memo: String - a String provided by the developer, shown to the user
/// - metadata: Value - an object provided by the developer for their own usage, serde_json::Value
/// - from_address: String - sender address of the blockchain transaction
/// - to_address: String - recipient address of the blockchain transaction
/// - direction: Direction - direction of the payment ("user_to_app" | "app_to_user")
/// - status: PaymentDTOStatus - Status flags representing the current state of this payment
///     - developer_approved: bool - Server-Side Approval (automatically approved for A2U payment)
///     - transaction_verified: bool - blockchain transaction verified
///     - developer_completed: bool - Server-Side Completion (handled by the create_payment! method)
///     - cancelled: bool - cancelled by the developer or by Pi Network
///     - user_cancelled: bool - cancelled by the user
/// - transaction: `Option<PaymentDTOTransaction>` - Blockchain transaction data. This is None if no transaction has been made yet
///     - txid: String - id of the blockchain transaction
///     - verified: bool - true if the transaction matches the payment, false otherwise
///     - _link: String - a link to the operation on the Pi Blockchain API
/// - created_at: String - payment's creation timestamp
/// - network: NetworkPassphrase - a network of the payment ("Pi Network" | "Pi Testnet")

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

/// See at `PaymentDTO`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDTOStatus {
    pub developer_approved: bool,
    pub transaction_verified: bool,
    pub developer_completed: bool,
    pub cancelled: bool,
    pub user_cancelled: bool,
}

/// See at `PaymentDTO`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDTOTransaction {
    pub txid: String,
    pub verified: bool,
    pub _link: String,
}

/// Reqwest client options
///
/// have base_url: String value, need to cleare `PiNetwork` struct, but since it's option also can be `None`
/// Usally have to keep at None, then the library will use `https://api.minepi.com` as the API base URL
/// But if for some reason the API url would change, we can give that also and the crate will keep working if nothing else changed.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqwestClientOptions {
    pub base_url: String,
}

/// Direction of the payment
///
/// Basically can be UserToApp or AppToUser and the json response from server ("user_to_app", "app_to_user") can be serialized in this enum.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "user_to_app")]
    UserToApp,
    #[serde(rename = "app_to_user")]
    AppToUser,
}

/// Network passphrase
///
/// Option for the `PiNetwork` struct, it's determining we will using testnet or mainnet
/// If we are creating the `Pi Network` struct with None, we will using the testnet as default

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkPassphrase {
    #[serde(rename = "Pi Network")]
    PiNetwork,
    #[serde(rename = "Pi Testnet")]
    PiTestnet,
}

/// Pi API's incomplete payments response serialized to this struct

#[derive(Debug, Deserialize, Serialize)]
pub struct IncompletePaymentResponse {
    pub incomplete_server_payments: Vec<PaymentDTO>,
}

/// Custom error wrapper for some possible error variants

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
