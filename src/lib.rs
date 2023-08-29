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

    // You can create an A2U payment using create_payment method. This method returns a payment identifier (payment id).
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

    // You can submit the payment to the Pi Blockchain using submit_payment method. This method builds a payment transaction and submits it to the Pi Blockchain for you. Once submitted, the method returns a transaction identifier (txid).
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

    // This method completes the payment in the Pi server.
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

    // This method returns a payment object if it exists.
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

    // This method cancels the payment in the Pi server.
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

    // This method returns the latest incomplete payment which your app has created, if present. Use this method to troubleshoot the following error: "You need to complete the ongoing payment first to create a new one."
    //
    // If a payment is returned by this method, you must follow one of the following 3 options:
    // - cancel the payment, if it is not linked with a blockchain transaction and you don't want to submit the transaction anymore
    // - submit the transaction and complete the payment
    // - if a blockchain transaction has been made, complete the payment
    //
    // If you do not know what this payment maps to in your business logic, you may use its metadata property to retrieve which business logic item it relates to. Remember that metadata is a required argument when creating a payment, and should be used as a way to link this payment to an item of your business logic.
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

    async fn get_horizon_client(network: NetworkPassphrase) -> Server {
        let server_url = match network {
            NetworkPassphrase::PiNetwork => "https://api.mainnet.minepi.com".to_string(),
            NetworkPassphrase::PiTestnet => "https://api.testnet.minepi.com".to_string(),
        };
        Server::new(server_url, None).unwrap()
    }

    async fn get_network_passphrase(network: NetworkPassphrase) -> String {
        let server_url = match network {
            NetworkPassphrase::PiNetwork => "Pi Network".to_string(),
            NetworkPassphrase::PiTestnet => "Pi Testnet".to_string(),
        };
        server_url
    }

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

    async fn submit_transaction(
        pi_horizon: Server,
        transaction: Transaction,
    ) -> Result<String, PiError> {
        let tx_response = pi_horizon.submit_transaction(transaction)?;
        Ok(tx_response.id)
    }
}
