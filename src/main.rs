use std::sync::LazyLock;

use chrono::{DateTime, FixedOffset};
use postgrest::Postgrest;
use uuid::Uuid;

mod config;

#[macro_use]
extern crate tracing;

static CFG: LazyLock<config::Config> = LazyLock::new(|| {
    use figment::*;

    Figment::new()
        .merge(figment::providers::Env::prefixed("ACCLE_"))
        .extract()
        .expect("could not parse config")
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().expect("no `.env` file");

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
    }
    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    let client = get_client();

    let account = CreateAccount {
        kind: AccountKind::Income,
        name: "First Student, Inc.".to_string(),
        description: Some("test".to_string()),
    };
    let resp = client
        .from("accounts")
        .insert(serde_json::to_string(&account).unwrap())
        .execute()
        .await?;

    info!("{:?}", resp.text().await?);

    Ok(())
}

fn get_client() -> postgrest::Postgrest {
    let client = postgrest::Postgrest::new(&CFG.postgrest_endpoint)
        .insert_header("apikey", &CFG.postgrest_apikey);

    if let Some(servicekey) = &CFG.postgrest_servicekey {
        client.insert_header("Authorization", format!("Bearer {servicekey}"))
    } else {
        client
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CreateAccount {
    kind: AccountKind,
    name: String,
    description: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum AccountKind {
    #[serde(rename = "EQUITY")]
    Equity,

    #[serde(rename = "ASSET")]
    Asset,

    #[serde(rename = "LIABILITY")]
    Liability,

    #[serde(rename = "INCOME")]
    Income,

    #[serde(rename = "EXPENSE")]
    Expense,
}

fn create_account(name: impl AsRef<str>, description: Option<impl AsRef<str>>, kind: AccountKind) {}

struct Ledger {
    client: Postgrest
}