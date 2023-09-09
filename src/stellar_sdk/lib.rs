//! # Stellar Sdk
//!
//! A lightweight Rust library for communicating with a Stellar Horizon server.
//!
//! ## Usage:
//!
//!
//! ```
//! use pi_rust::stellar_sdk::{endpoints::call_builder::CallBuilder, endpoints::server::Server, types::Asset, utils::{Direction, Endpoint}};
//!
//!     let s = String::from("https://horizon.stellar.org");
//!     let s = Server::new(s, None).expect("Cannot connect to insecure horizon server");
//!
//!     let my_acc = s
//!         .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
//!         .unwrap();
//!
//!     // Load transactions of an account
//!     let my_account_id = String::from("GAP2TJNW7NL52MPB36DZ2PB6PSIBEUEJXDG325BJQKUNDQBPKX3E2DLV");
//!     let my_txs = s
//!         .transactions()
//!         .order(Direction::Desc)
//!         .limit(2)
//!         .include_failed(false)
//!         .for_endpoint(Endpoint::Accounts(my_account_id))
//!         .call()
//!         .unwrap();
//!
//!     // Load trades of yXLM and XLM
//!     let y_xlm = Asset::new(
//!         String::from("yXLM"),
//!         String::from("GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55"),
//!     ).unwrap();
//!
//!     let native = Asset::native();
//!
//!     let xlm_trades = s
//!         .trades()
//!         .for_asset_pair(&y_xlm, &native)
//!         .limit(2)
//!         .call()
//!         .unwrap();
//!
//!     // Load USDC liquidity pools
//!     let usdc = Asset::new(
//!         String::from("USDC"),
//!         String::from("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
//!     ).unwrap();
//!     let usdc_liquidity_pools = s.liquidity_pools().for_assets(vec![usdc]).call().unwrap();
//!
//! ```

pub use crate::stellar_sdk::endpoints::call_builder::CallBuilder;
pub use crate::stellar_sdk::endpoints::server::Server;
pub use crate::stellar_sdk::endpoints::toml_resolver::StellarTomlResolver;
#[cfg(feature = "nacl")]
pub use crate::stellar_sdk::keypair::Keypair;
pub use crate::stellar_sdk::str_key::StrKey;

#[cfg(test)]
mod tests {
    use crate::stellar_sdk::{
        types::Asset,
        utils::{Direction, Endpoint},
        endpoints::call_builder::CallBuilder, endpoints::server::Server,
    };

    #[test]
    fn test_app() {
        let s = String::from("https://horizon.stellar.org");
        let s = Server::new(s, None).expect("Cannot connect to insecure horizon server");

        let _my_acc = s
            .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
            .unwrap();

        // Load transactions of an account
        let my_account_id =
            String::from("GAP2TJNW7NL52MPB36DZ2PB6PSIBEUEJXDG325BJQKUNDQBPKX3E2DLV");
        let _my_txs = s
            .transactions()
            .order(Direction::Desc)
            .limit(2)
            .include_failed(false)
            .for_endpoint(Endpoint::Accounts(my_account_id))
            .call()
            .unwrap();

        // Load trades of yXLM and XLM
        let y_xlm = Asset::new(
            String::from("yXLM"),
            String::from("GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55"),
        )
        .unwrap();

        let native = Asset::native();

        let _xlm_trades = s
            .trades()
            .for_asset_pair(&y_xlm, &native)
            .limit(2)
            .call()
            .unwrap();

        // Load USDC liquidity pools
        let usdc = Asset::new(
            String::from("USDC"),
            String::from("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
        )
        .unwrap();
        let _usdc_liquidity_pools = s.liquidity_pools().for_assets(vec![usdc]).call().unwrap();
    }
}
