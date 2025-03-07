
#[cfg(test)]
mod tests {
    use pi_rust::{types::*, PiNetwork};
    use serde_json::json;
    use std::env;

    fn setup() -> PiNetwork {
        dotenvy::dotenv().expect("Failed to load the env file");
        let pi_api_key: String = env::var("PI_API_KEY").expect("PI_API_KEY must be set");
        let wallet_private_seed: String = env::var("WALLET_PRIVATE_SEED").expect("WALLET_PRIVATE_SEED must be set");
        PiNetwork::new(pi_api_key, wallet_private_seed, None, None).unwrap()
    }


    #[tokio::test]
    async fn test_combined() {
        let mut pi = setup();

        let user_uid = env::var("USER_ID").expect("USER_ID must be set");

        println!("Complete payment test");
        let payment_data = PaymentArgs {
        amount: 0.1,
        memo: "Refund for apple pie".to_string(),
        metadata: json!({"productId": "apple-pie-1"}),
        uid: user_uid.clone()
        };

        let payment_id = pi.create_payment(payment_data.clone()).await.unwrap();
        let submit_payment_txid = pi.submit_payment(payment_id.clone()).await.unwrap();
        let complete_payment = pi.complete_payment(payment_id.clone(), submit_payment_txid).await.unwrap();

        assert_eq!(complete_payment.identifier, payment_id);

        println!("Cancel payment test");
        let payment_id2 = pi.create_payment(payment_data.clone()).await.unwrap();

        let get_payment2 = pi.get_payment(payment_id2.clone().to_string()).await.unwrap();

        let cancel_payment2 = pi.cancel_payment(payment_id2).await.unwrap();
        assert_eq!(get_payment2.identifier, cancel_payment2.identifier);

        println!("Looking for incomplete payment");
        let payment_data2 = PaymentArgs {
            amount: 2.1,
            memo: "Refund for apple pie".to_string(),
            metadata: json!({"productId": "apple-pie-1"}),
            uid: user_uid.clone()
        };

        let payment_id3 = pi.create_payment(payment_data2.clone()).await.unwrap();

        let incomplete_payment = pi.get_incomplete_server_payments().await.unwrap();
        let have_incomplete_vec = incomplete_payment.is_empty();

        let _cancel_payment = pi.cancel_payment(payment_id3).await;

        let incomplete_payment2 = pi.get_incomplete_server_payments().await.unwrap();
        let no_incomplete_vec = incomplete_payment2.is_empty();
        assert_ne!(have_incomplete_vec, no_incomplete_vec);
    }

    #[test]
    fn test_validate_seed_format() {
        let seed_valid = "SAFPHSUDCR3UUQX36MMRXJZBVZNKFP5OFOZSOLUWTT76QQUPKUUFNRNW";
        let seed_not_s = "WAFPHSUDCR3UUQX36MMRXJZBVZNKFP5OFOZSOLUWTT76QQUPKUUFNRNS";
        let seed_too_long = "SAFPHSUDCR3UUQX36MMRXJZBVZNKFP5OFOZSOLUWTT76QQUPKUUFNRNWG";
        let seed_too_short = "SAFPHSUDCR3UUQX36MMRXJZBVZNKFP5OFOZSOLUWTT76QQUPKUUFNR";
        let result_ok = PiNetwork::validate_seed_format(seed_valid);
        let result_err_not_s = PiNetwork::validate_seed_format(seed_not_s);
        let result_err_too_long = PiNetwork::validate_seed_format(seed_too_long);
        let result_err_too_short = PiNetwork::validate_seed_format(seed_too_short);
        assert_eq!(true, result_ok.is_ok());
        assert_eq!(true, result_err_not_s.is_err());
        assert_eq!(true, result_err_too_long.is_err());
        assert_eq!(true, result_err_too_short.is_err());
    }
}