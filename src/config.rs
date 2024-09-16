#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub postgrest_endpoint: String,
    pub postgrest_apikey: String,
    pub postgrest_servicekey: Option<String>,
}
