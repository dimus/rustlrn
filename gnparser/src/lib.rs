const HTTP_URL: &str = "https://parser.globalnames.org/api";

mod error;
mod method;

use crossbeam_channel::{Receiver, Sender};
use error::GNParserError;
pub use method::Method;
use reqwest::Url;
use serde_json::json;

#[derive(Debug, Clone, Default)]
pub struct GNParser {
    pub http_url: String,
    pub method: Method,
    pub batch_size: usize,
}

impl GNParser {
    pub fn new() -> Self {
        GNParser {
            http_url: HTTP_URL.to_owned(),
            batch_size: 500,
            ..Default::default()
        }
    }

    pub fn method(&mut self, m: Method) {
        self.method = m;
    }

    pub async fn parse(&self, inputs: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client.post(&self.http_url).json(&inputs).send().await?;
        println!("{}", res.text().await?);
        Ok(())
    }

    pub async fn parse_and_format(
        &self,
        inputs: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.parse(inputs).await?;
        Ok(())
    }
    pub fn parse_stream(&mut self, _in_r: Receiver<Vec<String>>, _out_s: Sender<Vec<String>>) {}
    pub fn format_outputs(&self, _outputs: Vec<String>, _is_first: bool) {}
}
