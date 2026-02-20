# xrpl-ws

A websocket client library for interacting with the xrp ledger's json rpc. At its core, the client maintains a single websocket connection, but is able to make atomic requests to the json rpc with a single function call, or make subscriptions with a function that returns a listener for incoming messages. These features are implemented using the websocket implementation by `tokio-tungstenite`, as well as using channels implemented by `tokio::sync`.

## Usage

```rust
use xrpl::{XrplClient, request};

#[tokio::main]
async fn main() {
    let account = "r..."; // an xrpl address
    let client = XrplClient::new("wss://xrpl.ws").await.unwrap();
    let server_info = client.request(request::AccountInfo { account: "r...".into(), ..Default::default() }).await;
    println!("{server_info}");
}
```

Subscriptions, on the other hand, are handled somewhat differently. In javascript, a subscription would look look something like:

```js
import { Client } from "xrpl";

async function main() {
  const client = new Client("wss://xrpl.ws");
  const subscription_res = await client.request({
    command: "subscribe",
    streams: ["ledger"],
  });
  console.log(subscription_res); // an acknowledgement message wiht some relevant data
  client.on("ledgerClosed", (msg) => console.log(msg)); // the actual messages, delivered via a callback
}
```

In this library, the same subscription would look something like this

```rust
async fn main() {
    let client = XrplClient::new("wss://xrpl.ws").await.unwrap();
    let (subscription_res, receiver) = client.request(subscription::LedgerClosed).await;
    println!("{subscription_res:?}"); // the acknowledgement message with data etc...
    loop {
          let msg = receiver.recv().await;
          match msg {
              Ok(msg) => println!("{msg:#?}"), // the actual messages
              Err(e) => {
                  eprintln!("{e:#?}");
                  break;
              }
          }
      }
}
```

The receiver is implemented using mpsc::broadcast, and as such can be quite trivially cloned through `receiver.resubscribe()`, or even moved into a different thread etc.
