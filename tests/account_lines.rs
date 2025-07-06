mod common;

use xrpl::*;
use common::*;

#[ignore]
#[tokio::test]
async fn test_account_lines() {
    let client = XrplClient::new(SERVER_URL).await.unwrap();
    let request = request::account_lines::AccountLinesRequest {
        account: TEST_ACCOUNT.to_string(),
        ledger_index: Some("validated".to_string()),
        ..Default::default()
    };

    let response = client.request(request).await.unwrap();
    let result =
        response.result().expect("Expected account currencies in response");
    let lines = &result.lines;

    assert!(lines.len() > 0);
}
