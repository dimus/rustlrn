const HTTP_URL: &str = "https://parser.globalnames.org/api";

mod error;
mod method;

use crossbeam_channel::{Receiver, Sender};
use error::GNParserError;
pub use method::Method;
use reqwest::Url;

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

    pub fn parse(&self, inputs: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let url_str = format!("{}?q={}", self.http_url, inputs.join("|"));
        let url = Url::parse(&url_str).unwrap();
        let req = client.get(url.as_str()).send();
        println!("{:#?}", req.unwrap().text());
        Ok(())
    }

    pub fn parse_and_format(&self, inputs: Vec<String>) {
        self.parse(inputs).unwrap();
    }
    pub fn parse_stream(&mut self, _in_r: Receiver<Vec<String>>, _out_s: Sender<Vec<String>>) {}
    pub fn format_outputs(&self, _outputs: Vec<String>, _is_first: bool) {}
}
