mod common;

use xrpl::*;
use common::*;

const TEST_ACCOUNT: &str = "raQshXKbbqYaQUcanRkwusEyV5eJdW9KpR";

#[ignore]
#[tokio::test]
async fn test_account_nfts() {
    let client = XrplClient::new(SERVER_URL).await.unwrap();
    let request = request::account_nfts::AccountNftsRequest {
        account: TEST_ACCOUNT.to_string(),
        ledger_index: Some("validated".to_string()),
        ..Default::default()
    };

    let response = client.request(request).await.unwrap();
    let result = response.result().expect("Expected account nfts in response");
    let nfts = &result.account_nfts;

    assert!(nfts.len() > 0);
}
