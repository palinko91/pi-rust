
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
    async fn test_create_a2u() {
        let mut pi = setup();
        let user_uid = "user_uid_of_your_app".to_string();
        let payment_data = PaymentArgs {
        amount: 1.0,
        memo: "Refund for apple pie".to_string(), // this is just an example
        metadata: json!({"productId": "apple-pie-1"}),
        uid: user_uid
        };
       
        let payment_id = pi.create_payment(payment_data).await;
        let result = payment_id.is_err();
        assert_eq!(true, result);
    }

    #[tokio::test]
    async fn test_submit_payment() {
        let mut pi = setup();
        let submit = pi.submit_payment("testpaymentid".to_string()).await;
        let result = submit.is_err();
        assert_eq!(true, result);
    }

    #[tokio::test]
    async fn test_complete_payment() {
        let mut pi = setup();
        let complete_payment = pi.complete_payment("testpaymentid".to_string(), "testtxid".to_string()).await;
        let result = complete_payment.is_err();
        assert_eq!(true, result);
    }

    #[tokio::test]
    async fn test_get_payment() {
        let mut pi = setup();
        let get_payment = pi.get_payment("testpaymentid".to_string()).await;
        let result = get_payment.is_err();
        assert_eq!(true, result);
    }

    #[tokio::test]
    async fn test_cancel_payment() {
        let mut pi = setup();
        let cancel_payment = pi.cancel_payment("testpaymentid".to_string()).await;
        let result = cancel_payment.is_err();
        assert_eq!(true, result);
    }

    #[tokio::test]
    async fn test_get_incomplete_server_payments() {
        let mut pi = setup();
        let incomplete_payment = pi.get_incomplete_server_payments().await.unwrap();
        let empty_vec = incomplete_payment.is_empty();
        assert_eq!(true, empty_vec);
    }

    fn test_validate_seed_format() {
        let seed_valid = "SAFPHSUDCR3UUQX36MMRXJZBVZNKFP5OFOZSOLUWTT76QQUPKUUFNRNW";
        let seed_not_s = "WAFPHSUDCR3UUQX36MMRXJZBVZNKFP5OFOZSOLUWTT76QQUPKUUFNRNS";
        let seed_too_long = "SAFPHSUDCR3UUQX36MMRXJZBVZNKFP5OFOZSOLUWTT76QQUPKUUFNRNWG";
        let result_ok = PiNetwork::validate_seed_format(seed_valid);
        let result_err_not_s = PiNetwork::validate_seed_format(seed_not_s);
        let result_err_too_long = PiNetwork::validate_seed_format(seed_too_long);
        assert_eq!(true, result_ok.is_ok());
        assert_eq!(true, result_err_not_s.is_err());
        assert_eq!(true, result_err_too_long.is_err());
    }
}