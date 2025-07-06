mod common;

use xrpl::*;
use common::*;

#[ignore]
#[tokio::test]
async fn test_account_currencies() {
    let client = XrplClient::new(SERVER_URL).await.unwrap();
    let request = request::account_currencies::AccountCurrenciesRequest {
        account: TEST_ACCOUNT.to_string(),
        ledger_index: Some("validated".to_string()),
        ..Default::default()
    };

    let response = client.request(request).await.unwrap();
    let result =
        response.result().expect("Expected account currencies in response");

    let send_currencies = &result.send_currencies;
    let receive_currencies = &result.receive_currencies;

    assert!(send_currencies.is_empty() == false);
    assert!(receive_currencies.is_empty() == false);
}
