mod common;

use xrpl::*;
use xrpl::request::account_objects::*;
use common::*;

macro_rules! test_account_object_type {
    ($test_name:ident, $variant:expr) => {
        #[ignore]
        #[tokio::test]
        async fn $test_name() {
            let client = XrplClient::new(SERVER_URL).await.unwrap();
            let request = AccountObjectsRequest {
                account: TEST_ACCOUNT.to_string(),
                ledger_index: Some("validated".to_string()),
                limit: Some(10),
                kind: Some($variant),
                ..Default::default()
            };

            let response = client
                .request(request)
                .await
                .expect(&format!("{}", stringify!($variant)));
            response.result().expect(&format!(
                "Expected {} in response",
                stringify!($variant)
            ));
        }
    };
}

test_account_object_type!(bridge, AccountObjectRequestType::Bridge);
test_account_object_type!(check, AccountObjectRequestType::Check);
test_account_object_type!(deposit, AccountObjectRequestType::DepositPreauth);
test_account_object_type!(escrow, AccountObjectRequestType::Escrow);
test_account_object_type!(mptoken, AccountObjectRequestType::MPToken);
test_account_object_type!(nft_offer, AccountObjectRequestType::NFTokenOffer);
test_account_object_type!(nft_page, AccountObjectRequestType::NFTokenPage);
test_account_object_type!(offer, AccountObjectRequestType::Offer);
test_account_object_type!(paychannel, AccountObjectRequestType::PayChannel);
test_account_object_type!(state, AccountObjectRequestType::RippleState);
test_account_object_type!(signer_list, AccountObjectRequestType::SignerList);
test_account_object_type!(ticket, AccountObjectRequestType::Ticket);
