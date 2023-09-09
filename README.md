# Pi Network - Rust server-side package

This is a non-official Pi Network Rust crate you can use to integrate the Pi Network apps platform with a Rust backend application.

## Install

Install this package as a dependency of your app:

```shell
# With cargo:
cargo add pi_rust

# With toml file:
pi_rust = "0.1.0"
```

## Example

1. Initialize the SDK
```rust
use pi_rust::{PiNetwork, types::{ReqwestClientOptions, NetworkPassphrase}};

dotenvy::dotenv().expect("Failed to load the env file");

// DO NOT expose these values to public
let pi_api_key: String = env::var("PI_API_KEY").expect("PI_API_KEY must be set");
let wallet_private_seed: String = env::var("WALLET_PRIVATE_SEED").expect("WALLET_PRIVATE_SEED must be set"); // starts with S
// Important to note if you putting None, None, the application will run on testnet and api.minepi.com
// If you need mainnet do this
// use pi_rust::{types::NetworkPassphrase, PiNetwork};
// let pi = PiNetwork::new(pi_api_key, wallet_private_seed, Some(NetworkPassphrase::PiNetwork), None).unwrap();
// ReqwestClientOptions can be set also like Some(ReqwestClientOptions{base_url:"String".to_string()})
let pi = PiNetwork::new(pi_api_key, wallet_private_seed, None, None).unwrap();
```

2. Create an A2U payment

Make sure to store your payment data in your database. Here's an example of how you could keep track of the data.
Consider this a database table example.

| uid | product_id | amount | memo | payment_id | txid |
| :---: | :---: | :---: | :---: | :---: | :---: |
| `userUid` | apple-pie-1 | 3.14 | Refund for apple pie | NULL | NULL |

```rust
//  Get the user_uid from the Frontend
let user_uid = "user_uid_of_your_app".to_string();
let payment_data = PaymentArgs {
  amount: 1.0,
  memo: "Refund for apple pie".to_string(), // this is just an example
  metadata: json!({productId: "apple-pie-1"}),
  uid: user_uid
};
// It is critical that you store payment_id in your database
// so that you don't double-pay the same user, by keeping track of the payment.
let payment_id = pi.create_payment(payment_data).await;
```

3. Store the `payment_id` in your database

After creating the payment, you'll get `payment_id`, which you should be storing in your database.

| uid | product_id | amount | memo | payment_id | txid |
| :---: | :---: | :---: | :---: | :---: | :---: |
| `userUid` | apple-pie-1 | 3.14 | Refund for apple pie | `payment_id` | NULL |

4. Submit the payment to the Pi Blockchain
```rust
// It is strongly recommended that you store the txid along with the payment_id you stored earlier for your reference.
let txid = pi.submit_payment(payment_id).await;
```

5. Store the txid in your database

Similarly as you did in step 3, keep the txid along with other data.

| uid | product_id | amount | memo | payment_id | txid |
| :---: | :---: | :---: | :---: | :---: | :---: |
| `userUid` | apple-pie-1 | 3.14 | Refund for apple pie | `payment_id` | `txid` |

6. Complete the payment
```rust
let completed_payment = pi.complete_payment(payment_id, txid).await;
```

## Overall flow for A2U (App-to-User) payment

To create an A2U payment using the Pi Rust SDK, here's an overall flow you need to follow:

1. Initialize the SDK
> You'll be initializing the SDK with the Pi API Key of your app and the Private Seed of your app wallet, the network passphrase and the request url if needed.

2. Create an A2U payment
> You can create an A2U payment using `create_payment` method. This method returns a payment identifier (payment id).

3. Store the payment id in your database
> It is critical that you store the payment id, returned by `create_payment` method, in your database so that you don't double-pay the same user, by keeping track of the payment.

4. Submit the payment to the Pi Blockchain
> You can submit the payment to the Pi Blockchain using `submit_payment` method. This method builds a payment transaction and submits it to the Pi Blockchain for you. Once submitted, the method returns a transaction identifier (txid).

5. Store the txid in your database
> It is strongly recommended that you store the txid along with the payment id you stored earlier for your reference.

6. Complete the payment
> After checking the transaction with the txid you obtained, you must complete the payment, which you can do with `complete_payment` method. Upon completing, the method returns the payment object. Check the `status` field to make sure everything looks correct.

## SDK Reference

This section shows you a list of available methods.
### `create_payment`

This method creates an A2U payment.

- Required parameter: `PaymentArgs`

You need to provide 4 different data and pass them as a single object to this method.
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentArgs {
    pub amount: f64, // the amount of Pi you're paying to your user, always decimal so if 1 Pi then 1.0 should be entered
    pub memo: String, // a short memo that describes what the payment is about
    pub metadata: Value, // an arbitrary object that you can attach to this payment. This is for your own use. You should use this object as a way to link this payment with your internal business logic.
    pub uid: String, // a user uid of your app. You should have access to this value if a user has authenticated on your app.
}
```

- Return value: `a payment identifier (payment_id: String)`

### `submit_payment`

This method creates a payment transaction and submits it to the Pi Blockchain.

- Required parameter: `payment_id`
- Return value: `a transaction identifier (txid: String)`

### `complete_payment`

This method completes the payment in the Pi server.

- Required parameter: `payment_id, txid`
- Return value: `a payment object (payment: PaymentDTO)`

The method return a payment struct with the following fields:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDTO {
  // Payment data:
    pub identifier: String, // payment identifier
    pub user_uid: String, // user's app-specific ID
    pub amount: f64, // payment amount
    pub memo: String, // a String provided by the developer, shown to the user
    pub metadata: Value, // an object provided by the developer for their own usage
    pub from_address: String, // sender address of the blockchain transaction
    pub to_address: String, // recipient address of the blockchain transaction
    pub direction: Direction, // direction of the payment ("user_to_app" | "app_to_user")
    pub status: PaymentDTOStatus, // Status flags representing the current state of this payment
    pub transaction: Option<PaymentDTOTransaction>, // Blockchain transaction data. This is None if no transaction has been made yet
    pub created_at: String, // payment's creation timestamp
    pub network: NetworkPassphrase, // a network of the payment ("Pi Network" | "Pi Testnet")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDTOStatus {
    pub developer_approved: bool, // Server-Side Approval (automatically approved for A2U payment)
    pub transaction_verified: bool, // blockchain transaction verified
    pub developer_completed: bool, // Server-Side Completion (handled by the create_payment! method)
    pub cancelled: bool, // cancelled by the developer or by Pi Network
    pub user_cancelled: bool, // cancelled by the user
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDTOTransaction {
    pub txid: String, // id of the blockchain transaction
    pub verified: bool, // true if the transaction matches the payment, false otherwise
    pub _link: String, // a link to the operation on the Pi Blockchain API
}
```

### `get_payment`

This method returns a payment object if it exists.

- Required parameter: `payment_id`
- Return value: `a payment object (payment: PaymentDTO)`

### `cancel_payment`

This method cancels the payment in the Pi server.

- Required parameter: `payment_id`
- Return value: `a payment object (payment: PaymentDTO)`

### `get_incomplete_server_payments`

This method returns the latest incomplete payment which your app has created, if present. Use this method to troubleshoot the following error: "You need to complete the ongoing payment first to create a new one."

- Required parameter: `nothing`
- Return value: `a vector which contains 0 or 1 payment object (payments: Vec<PaymentDTO>)`

If a payment is returned by this method, you must follow one of the following 3 options:

1. cancel the payment, if it is not linked with a blockchain transaction and you don't want to submit the transaction anymore

2. submit the transaction and complete the payment

3. if a blockchain transaction has been made, complete the payment

If you do not know what this payment maps to in your business logic, you may use its `metadata` property to retrieve which business logic item it relates to. Remember that `metadata` is a required argument when creating a payment, and should be used as a way to link this payment to an item of your business logic.

## Troubleshooting

### Error when creating a payment: "You need to complete the ongoing payment first to create a new one."

See documentation for the `get_incomplete_server_payments` above.
