use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct LightningAddress {
    username: String,
    domain: String,
    callback: String,
}

impl LightningAddress {
    pub async fn new(lnaddress: &str) -> Self {
        let (username, domain) = parse_lnaddress(lnaddress);
        let mut address = LightningAddress {
            username: username.to_string(),
            domain: domain.to_string(),
            callback: String::new(),
        };
        address.validate_and_set_callback().await;
        address
    }

    async fn validate_and_set_callback(&mut self) {
        let well_known_response = self.get_well_known_response().await;
        // Add your validation logic here. For example:
        if well_known_response.status == "OK" {
            self.callback = well_known_response.callback;
        } else {
            panic!("Invalid well-known response");
        }
    }

    async fn get_well_known_response(&self) -> WellKnownResponse {
        let client = Client::new();
        let res = client
            .get(format!(
                "https://{}/.well-known/lnurlp/{}",
                self.domain, self.username
            ))
            .send()
            .await
            .expect("Failed to send request");

        serde_json::from_str(&res.text().await.unwrap()).expect("Failed to parse response")
    }

    pub async fn get_invoice(&self) -> String {
        let client = Client::new();
        let callback_res = client
            .get(format!("{}?amount=1000", self.callback))
            .send()
            .await
            .expect("Failed to send callback request");

        let response: CallbackResponse = serde_json::from_str(&callback_res.text().await.unwrap())
            .expect("Failed to parse callback response");

        response.pr // assuming pr field contains the invoice
    }
}

fn parse_lnaddress(lnaddress: &str) -> (&str, &str) {
    let mut parts = lnaddress.split('@');
    let username = parts.next().expect("Invalid LNADDRESS");
    let domain = parts.next().expect("Invalid LNADDRESS");
    (username, domain)
}

#[derive(Debug, Deserialize)]
struct PayerDataDetails {
    mandatory: bool,
}

#[derive(Debug, Deserialize)]
struct PayerData {
    name: PayerDataDetails,
    email: PayerDataDetails,
    pubkey: PayerDataDetails,
}

#[derive(Debug, Deserialize)]
struct WellKnownResponse {
    status: String,
    tag: String,
    commentAllowed: u8,
    callback: String,
    metadata: String,
    minSendable: i64,
    maxSendable: i64,
    payerData: PayerData,
    nostrPubkey: String,
    allowsNostr: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct SuccessAction {
    tag: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CallbackResponse {
    status: String,
    successAction: SuccessAction,
    verify: String,
    routes: Vec<String>,
    pr: String,
}
