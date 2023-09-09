//! 1. Initialize the SDK
//! ```
//! use pi_rust::{PiNetwork, types::{ReqwestClientOptions, NetworkPassphrase}};
//! 
//! dotenvy::dotenv().expect("Failed to load the env file");
//! 
//! // DO NOT expose these values to public
//! let pi_api_key: String = env::var("PI_API_KEY").expect("PI_API_KEY must be set");
//! let wallet_private_seed: String = env::var("WALLET_PRIVATE_SEED").expect("WALLET_PRIVATE_SEED must be set"); // starts with S
//! // Important to note if you putting None, None, the application will run on testnet and api.minepi.com
//! // If you need mainnet do this
//! // use pi_rust::{types::NetworkPassphrase, PiNetwork};
//! // let pi = PiNetwork::new(pi_api_key, wallet_private_seed, Some(NetworkPassphrase::PiNetwork), None).unwrap();
//! // ReqwestClientOptions can be set also like Some(ReqwestClientOptions{base_url:"String".to_string()})
//! let pi = PiNetwork::new(pi_api_key, wallet_private_seed, None, None).unwrap();
//! ```
//! 
//! 2. Create an A2U payment
//! 
//! Make sure to store your payment data in your database. Here's an example of how you could keep track of the data.
//! Consider this a database table example.
//! 
//! | uid | product_id | amount | memo | payment_id | txid |
//! | :---: | :---: | :---: | :---: | :---: | :---: |
//! | `userUid` | apple-pie-1 | 3.14 | Refund for apple pie | NULL | NULL |
//! 
//! ```
//! //  Get the user_uid from the Frontend
//! let user_uid = "user_uid_of_your_app".to_string();
//! let payment_data = PaymentArgs {
//!   amount: 1.0,
//!   memo: "Refund for apple pie".to_string(), // this is just an example
//!   metadata: json!({productId: "apple-pie-1"}),
//!   uid: user_uid
//! };
//! // It is critical that you store payment_id in your database
//! // so that you don't double-pay the same user, by keeping track of the payment.
//! let payment_id = pi.create_payment(payment_data).await;
//! ```
//! 
//! 3. Store the `payment_id` in your database
//! 
//! After creating the payment, you'll get `payment_id`, which you should be storing in your database.
//! 
//! | uid | product_id | amount | memo | payment_id | txid |
//! | :---: | :---: | :---: | :---: | :---: | :---: |
//! | `userUid` | apple-pie-1 | 3.14 | Refund for apple pie | `payment_id` | NULL |
//! 
//! 4. Submit the payment to the Pi Blockchain
//! ```
//! // It is strongly recommended that you store the txid along with the payment_id you stored earlier for your reference.
//! let txid = pi.submit_payment(payment_id).await;
//! ```
//! 
//! 5. Store the txid in your database
//! 
//! Similarly as you did in step 3, keep the txid along with other data.
//! 
//! | uid | product_id | amount | memo | payment_id | txid |
//! | :---: | :---: | :---: | :---: | :---: | :---: |
//! | `userUid` | apple-pie-1 | 3.14 | Refund for apple pie | `payment_id` | `txid` |
//! 
//! 6. Complete the payment
//! ```
//! let completed_payment = pi.complete_payment(payment_id, txid).await;
//! ```
//! 
//! ## Overall flow for A2U (App-to-User) payment
//! 
//! To create an A2U payment using the Pi Rust SDK, here's an overall flow you need to follow:
//! 
//! 1. Initialize the SDK
//! > You'll be initializing the SDK with the Pi API Key of your app and the Private Seed of your app wallet, the network passphrase and the request url if needed.
//! 
//! 2. Create an A2U payment
//! > You can create an A2U payment using `create_payment` method. This method returns a payment identifier (payment id).
//! 
//! 3. Store the payment id in your database
//! > It is critical that you store the payment id, returned by `create_payment` method, in your database so that you don't double-pay the same user, by keeping track of the payment.
//! 
//! 4. Submit the payment to the Pi Blockchain
//! > You can submit the payment to the Pi Blockchain using `submit_payment` method. This method builds a payment transaction and submits it to the Pi Blockchain for you. Once submitted, the method returns a transaction identifier (txid).
//! 
//! 5. Store the txid in your database
//! > It is strongly recommended that you store the txid along with the payment id you stored earlier for your reference.
//! 
//! 6. Complete the payment
//! > After checking the transaction with the txid you obtained, you must complete the payment, which you can do with `complete_payment` method. Upon completing, the method returns the payment object. Check the `status` field to make sure everything looks correct.
//! 
//! ## SDK Reference
//! 
//! This section shows you a list of available methods.
//! ### `create_payment`
//! 
//! This method creates an A2U payment.
//! 
//! - Required parameter: `PaymentArgs`
//! 
//! You need to provide 4 different data and pass them as a single object to this method.
//! ```
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! pub struct PaymentArgs {
//!     pub amount: f64, // the amount of Pi you're paying to your user, always decimal so if 1 Pi then 1.0 should be entered
//!     pub memo: String, // a short memo that describes what the payment is about
//!     pub metadata: Value, // an arbitrary object that you can attach to this payment. This is for your own use. You should use this object as a way to link this payment with your internal business logic.
//!     pub uid: String, // a user uid of your app. You should have access to this value if a user has authenticated on your app.
//! }
//! ```
//! 
//! - Return value: `a payment identifier (payment_id: String)`
//! 
//! ### `submit_payment`
//! 
//! This method creates a payment transaction and submits it to the Pi Blockchain.
//! 
//! - Required parameter: `payment_id`
//! - Return value: `a transaction identifier (txid: String)`
//! 
//! ### `complete_payment`
//! 
//! This method completes the payment in the Pi server.
//! 
//! - Required parameter: `payment_id, txid`
//! - Return value: `a payment object (payment: PaymentDTO)`
//! 
//! The method return a payment struct with the following fields:
//! 
//! ```
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! pub struct PaymentDTO {
//!   // Payment data:
//!     pub identifier: String, // payment identifier
//!     pub user_uid: String, // user's app-specific ID
//!     pub amount: f64, // payment amount
//!     pub memo: String, // a String provided by the developer, shown to the user
//!     pub metadata: Value, // an object provided by the developer for their own usage
//!     pub from_address: String, // sender address of the blockchain transaction
//!     pub to_address: String, // recipient address of the blockchain transaction
//!     pub direction: Direction, // direction of the payment ("user_to_app" | "app_to_user")
//!     pub status: PaymentDTOStatus, // Status flags representing the current state of this payment
//!     pub transaction: Option<PaymentDTOTransaction>, // Blockchain transaction data. This is None if no transaction has been made yet
//!     pub created_at: String, // payment's creation timestamp
//!     pub network: NetworkPassphrase, // a network of the payment ("Pi Network" | "Pi Testnet")
//! }
//! 
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! pub struct PaymentDTOStatus {
//!     pub developer_approved: bool, // Server-Side Approval (automatically approved for A2U payment)
//!     pub transaction_verified: bool, // blockchain transaction verified
//!     pub developer_completed: bool, // Server-Side Completion (handled by the create_payment! method)
//!     pub cancelled: bool, // cancelled by the developer or by Pi Network
//!     pub user_cancelled: bool, // cancelled by the user
//! }
//! 
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! pub struct PaymentDTOTransaction {
//!     pub txid: String, // id of the blockchain transaction
//!     pub verified: bool, // true if the transaction matches the payment, false otherwise
//!     pub _link: String, // a link to the operation on the Pi Blockchain API
//! }
//! ```
//! 
//! ### `get_payment`
//! 
//! This method returns a payment object if it exists.
//! 
//! - Required parameter: `payment_id`
//! - Return value: `a payment object (payment: PaymentDTO)`
//! 
//! ### `cancel_payment`
//! 
//! This method cancels the payment in the Pi server.
//! 
//! - Required parameter: `payment_id`
//! - Return value: `a payment object (payment: PaymentDTO)`
//! 
//! ### `get_incomplete_server_payments`
//! 
//! This method returns the latest incomplete payment which your app has created, if present. Use this method to troubleshoot the following error: "You need to complete the ongoing payment first to create a new one."
//! 
//! - Required parameter: `nothing`
//! - Return value: `a vector which contains 0 or 1 payment object (payments: Vec<PaymentDTO>)`
//! 
//! If a payment is returned by this method, you must follow one of the following 3 options:
//! 
//! 1. cancel the payment, if it is not linked with a blockchain transaction and you don't want to submit the transaction anymore
//! 
//! 2. submit the transaction and complete the payment
//! 
//! 3. if a blockchain transaction has been made, complete the payment
//! 
//! If you do not know what this payment maps to in your business logic, you may use its `metadata` property to retrieve which business logic item it relates to. Remember that `metadata` is a required argument when creating a payment, and should be used as a way to link this payment to an item of your business logic.
//! 
//! ## Troubleshooting
//! 
//! ### Error when creating a payment: "You need to complete the ongoing payment first to create a new one."
//! 
//! See documentation for the `get_incomplete_server_payments` above.
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod types;
use reqwest::{header, Client, StatusCode};
use serde_json::{json, Value};
use std::str::FromStr;
use stellar_base::{
    amount::{Amount, Stroops},
    asset::Asset,
    crypto::MuxedAccount,
    memo::Memo,
    operations::Operation,
    transaction::Transaction,
    Network, PublicKey, KeyPair,
};
use stellar_sdk::{Keypair, Server, types::Account};
use types::*;

/// Creating the reqwest client for the api call with the necessary header authentication
fn get_reqwest_client(api_key: String) -> Client {
    let mut headers = header::HeaderMap::new();
    let mut auth_value =
        header::HeaderValue::from_str(format!("Key {}", api_key).as_str()).unwrap();
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let client = Client::builder()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .unwrap();

    client
}

/// The `PiNetwork` struct containing every iformation for API authorization and the wallet handling
/// Have to initialized once and we can call all the methods on this struct.
/// If we want to handle more payments concurrently, we have to initialize one struct for every case.
/// The struct can store one payment flow and focus on that.
pub struct PiNetwork {
    pub api_key: String,
    pub my_key_pair: Keypair,
    pub network_passphrase: Option<NetworkPassphrase>,
    pub current_payment: Option<PaymentDTO>,
    pub reqwest_options: Option<ReqwestClientOptions>,
}

impl PiNetwork {
    // Creating new PiNetwork struct for the crate's user
    pub fn new(
        api_key: String,
        wallet_private_seed: String,
        network_passphrase: Option<NetworkPassphrase>,
        options: Option<ReqwestClientOptions>,
    ) -> Result<Self, PiError> {
        // Validating the seed format for common errors if erroring out returning error for the new function otherwise we continue the code
        Self::validate_seed_format(&wallet_private_seed)?;
        let my_key_pair = Keypair::from_secret_key(&wallet_private_seed);

        // Matching if everything went ok then we are returning struct if the key pair generation errored then we returning that error
        match my_key_pair {
            Ok(my_key_pair) => {
                return Ok(PiNetwork {
                    api_key,
                    my_key_pair,
                    network_passphrase,
                    current_payment: None,
                    reqwest_options: options,
                })
            }
            Err(e) => return Err(PiError::Message(format!("{:?}", e))),
        }
    }

    /// You can create an A2U payment using create_payment method. This method returns a payment identifier (payment id).
    pub async fn create_payment(&mut self, payment_data: PaymentArgs) -> Result<String, PiError> {
        let client = get_reqwest_client(self.api_key.clone());
        let body = json!({ "payment": payment_data });
        let url = match &self.reqwest_options {
            Some(options) => options.base_url.clone(),
            None => "https://api.minepi.com".to_string(),
        };

        let response = client
            .post(format!("{url}/v2/payments"))
            .json(&body)
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let response_data: Value = response.json().await?;
            let payment_dto: PaymentDTO = serde_json::from_value(response_data.clone())?;
            self.current_payment = Some(payment_dto.clone());

            Ok(payment_dto.identifier)
        } else {
            Err(PiError::Message(format!("Error, message from API: {:?}", response.text().await)))
        }
    }

    /// You can submit the payment to the Pi Blockchain using submit_payment method. This method builds a payment transaction and submits it to the Pi Blockchain for you. Once submitted, the method returns a transaction identifier (txid).
    pub async fn submit_payment(&mut self, payment_id: String) -> Result<String, PiError> {
        if let Some(current_payment) = self.current_payment.clone() {
            if current_payment.identifier != payment_id {
                self.current_payment = Some(self.get_payment(payment_id.clone()).await?);
                if let Some(transaction) = current_payment.transaction {
                    let tx_id = transaction.txid;
                    return Err(PiError::Message(format!(
                        "This payment already has a linked txid: Payment ID: {}, TX ID: {}",
                        payment_id, tx_id
                    )));
                }
            }

            let payment = self.current_payment.clone().unwrap();
            let amount = payment.amount.clone();
            let payment_identifier = payment.identifier.clone();
            let from_address = payment.from_address.clone();
            let to_address = payment.to_address.clone();
            let network = payment.network.clone();

            let pi_horizon = PiNetwork::get_horizon_client(network).await;

            let transaction_data = TransactionData {
                amount,
                payment_identifier,
                from_address,
                to_address,
            };

            let transaction = self
                .build_a2u_transaction(pi_horizon.clone(), transaction_data)
                .await?;

            let txid = PiNetwork::submit_transaction(pi_horizon.clone(), transaction).await?;

            self.current_payment = None;
            Ok(txid)
        } else {
            Err(PiError::Message("No current payment available".to_string()))
        }
    }

    /// This method completes the payment in the Pi server.
    pub async fn complete_payment(
        &mut self,
        payment_id: String,
        tx_id: String,
    ) -> Result<PaymentDTO, PiError> {
        let client = get_reqwest_client(self.api_key.clone());
        let url = match &self.reqwest_options {
            Some(options) => options.base_url.clone(),
            None => "https://api.minepi.com".to_string(),
        };
        let response = client
            .post(format!("{url}/v2/payments/{payment_id}/complete?txid={tx_id}"))
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let response_data: Value = response.json().await?;
            let payment_dto: PaymentDTO = serde_json::from_value(response_data.clone())?;
            self.current_payment = None;

            Ok(payment_dto)
        } else {
            Err(PiError::Message(format!("Error, message from API: {:?}", response.text().await)))
        }
    }

    /// This method returns a payment object based on the payment ID if it exists.
    pub async fn get_payment(&mut self, payment_id: String) -> Result<PaymentDTO, PiError> {
        let client = get_reqwest_client(self.api_key.clone());
        let url = match &self.reqwest_options {
            Some(options) => options.base_url.clone(),
            None => "https://api.minepi.com".to_string(),
        };
        let response = client
            .get(format!("{url}/v2/payments/{payment_id}"))
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let response_data: Value = response.json().await?;
            let payment: PaymentDTO = serde_json::from_value(response_data)?;

            Ok(payment)
        } else {
            Err(PiError::Message(format!("Error, message from API: {:?}", response.text().await)))
        }
    }

    /// This method cancels the payment in the Pi server.
    pub async fn cancel_payment(&mut self, payment_id: String) -> Result<PaymentDTO, PiError> 
    {
        let client = get_reqwest_client(self.api_key.clone());
        let url = match &self.reqwest_options {
            Some(options) => options.base_url.clone(),
            None => "https://api.minepi.com".to_string(),
        };
        let response = client
            .post(format!("{url}/v2/payments/{payment_id}/cancel"))
            .send()
            .await.unwrap();

        if response.status() == StatusCode::OK {
            let response_data: PaymentDTO = response.json().await?;
            Ok(response_data)
        } else {
            Err(PiError::Message(format!("Error, message from API: {:?}", response.text().await)))
        }
    }

    /// This method returns the latest incomplete payment which your app has created, if present. Use this method to troubleshoot the following error: "You need to complete the ongoing payment first to create a new one."
    ///
    /// If a payment is returned by this method, you must follow one of the following 3 options:
    /// - cancel the payment, if it is not linked with a blockchain transaction and you don't want to submit the transaction anymore
    /// - submit the transaction and complete the payment
    /// - if a blockchain transaction has been made, complete the payment
    ///
    /// If you do not know what this payment maps to in your business logic, you may use its metadata property to retrieve which business logic item it relates to. Remember that metadata is a required argument when creating a payment, and should be used as a way to link this payment to an item of your business logic.
    pub async fn get_incomplete_server_payments(&self) -> Result<Vec<PaymentDTO>, PiError> {
        let client = get_reqwest_client(self.api_key.clone());
        let url = match &self.reqwest_options {
            Some(options) => options.base_url.clone(),
            None => "https://api.minepi.com".to_string(),
        };
        let response = client
            .get(format!("{url}/v2/payments/incomplete_server_payments"))
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let response_data: IncompletePaymentResponse = response.json().await?;
            let payment_vec: Vec<PaymentDTO> = response_data.incomplete_server_payments;
            Ok(payment_vec)
        } else {
            Err(PiError::Message(format!("Error, message from API: {:?}", response.text().await)))
        }
    }

    /// Validating the seed format, trying to filter out invalid wallet secret seeds
    pub fn validate_seed_format(seed: &str) -> Result<(), PiError> {
        if !seed.starts_with("S") {
            return Err(PiError::Message(
                "Wallet private seed must start with 'S'".to_string(),
            ));
        }
        if seed.len() != 56 {
            return Err(PiError::Message(
                "Wallet private seed must be 56 characters long".to_string(),
            ));
        }
        Ok(())
    }

    /// Based on which network we want to use it returning the correct URI
    async fn get_horizon_client(network: NetworkPassphrase) -> Server {
        let server_url = match network {
            NetworkPassphrase::PiNetwork => "https://api.mainnet.minepi.com".to_string(),
            NetworkPassphrase::PiTestnet => "https://api.testnet.minepi.com".to_string(),
        };
        Server::new(server_url, None).unwrap()
    }

    // Based on which network we wnat to use it retunring the network's passphrase
    async fn get_network_passphrase(network: NetworkPassphrase) -> String {
        let server_url = match network {
            NetworkPassphrase::PiNetwork => "Pi Network".to_string(),
            NetworkPassphrase::PiTestnet => "Pi Testnet".to_string(),
        };
        server_url
    }

    /// Building app to user trasanction
    async fn build_a2u_transaction(
        &self,
        pi_horizon: Server,
        transaction_data: TransactionData,
    ) -> Result<Transaction, PiError> {
        if transaction_data.from_address != self.my_key_pair.public_key() {
            return Err(PiError::Message(
                "You should use a private seed of your app wallet!".to_string(),
            ));
        }

        let my_account: Account = pi_horizon.load_account(&self.my_key_pair.public_key())?;
        let base_fee_string = pi_horizon.fetch_base_fee()?;
        let base_fee_i64 = base_fee_string.parse::<i64>()?;
        let base_fee = Stroops::new(base_fee_i64);

        let amount_str = transaction_data.amount.clone().to_string();
        let destination_account_public_key = PublicKey::from_account_id(&transaction_data.to_address.clone());
        let destination_account_muxed: MuxedAccount = match destination_account_public_key {
            Ok(account) => account.into(),
            Err(e) => {
                return Err(PiError::Message(format!(
                    "Can't make muxed account from the given destination account ID! {:?}",
                    e
                )));
            }
        };
        
        let payment_operation = Operation::new_payment()
            .with_destination(destination_account_muxed.clone())
            .with_amount(Amount::from_str(&amount_str).unwrap())
            .unwrap()
            .with_asset(Asset::new_native())
            .build()
            .unwrap();
       
        let sequence = my_account.sequence.clone().parse::<i64>().unwrap() + 1; // Getting the current sequence of the account and adding 1 to it

        let source_account_keypair: KeyPair = self.my_key_pair.clone().into();
        let source_account_public_key = source_account_keypair.public_key();
        let source_account_muxed: MuxedAccount = source_account_public_key.clone().into();
       
        let mut transaction = Transaction::builder(source_account_muxed, sequence, base_fee)
            .with_memo(Memo::Text(transaction_data.payment_identifier.clone()))
            .add_operation(payment_operation)
            .into_transaction()
            .unwrap();
      
        // If the user gave us the network passphrase we are using that if he not then going with testnet as default
        let network_passphrase_enum: NetworkPassphrase = match &self.network_passphrase {
            Some(passphrase) => passphrase.clone(),
            None => NetworkPassphrase::PiTestnet,
        };

        // Signing the transaction
        let _ = transaction.sign(
            &source_account_keypair,
            &Network::new(PiNetwork::get_network_passphrase(network_passphrase_enum).await),
        );
        Ok(transaction)
    }

    /// Submitting the built transaction to the blockchain
    async fn submit_transaction(
        pi_horizon: Server,
        transaction: Transaction,
    ) -> Result<String, PiError> {
        let tx_response = pi_horizon.submit_transaction(transaction)?;
        Ok(tx_response.id)
    }
}
