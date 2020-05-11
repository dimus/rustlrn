const HTTP_URL: &str = "https://parser.globalnames.org/api";

mod error;
mod method;
mod sci_name;

use crossbeam_channel::{Receiver, Sender};
use error::GNParserError;
pub use method::Method;
pub use sci_name::{Canonical, SciName};

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

    pub async fn parse(&self, inputs: Vec<String>) -> Result<(), surf::Exception> {
        let mut res = surf::post(&self.http_url).body_json(&inputs)?.await?;

        let sci_name: Vec<SciName> = res.body_json().await?;
        println!("{:#?}", sci_name);
        Ok(())
    }

    pub async fn parse_and_format(&self, inputs: Vec<String>) -> Result<(), surf::Exception> {
        self.parse(inputs).await?;
        Ok(())
    }
    pub fn parse_stream(&mut self, _in_r: Receiver<Vec<String>>, _out_s: Sender<Vec<String>>) {}
    pub fn format_outputs(&self, _outputs: Vec<String>, _is_first: bool) {}
}
