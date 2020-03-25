use serde::Deserialize;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io;

const BHL_URL: &str = "https://beta.biodiversitylibrary.org/item/79357#page/";

#[derive(Deserialize, Debug, Clone, Default)]
struct Page {
    #[serde(skip_deserializing)]
    page: u32,
    metadata: Metadata,
    names: Option<Vec<Name>>,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct Metadata {
    total_words: u32,
    total_candidates: u32,
    total_names: u32,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct Name {
    #[serde(rename(deserialize = "type"))]
    name_type: String,
    verbatim: String,
    name: String,
    odds: f64,
    annotation: String,
    verification: Verification,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct Verification {
    #[serde(rename(deserialize = "BestResult"))]
    best_result: BestResult,
    #[serde(rename(deserialize = "dataSourcesNum"))]
    data_sources_num: Option<u8>,
    #[serde(rename(deserialize = "dataSourceQuality"))]
    data_source_quality: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct BestResult {
    #[serde(rename(deserialize = "dataSourceId"))]
    data_source_id: Option<u16>,
    #[serde(rename(deserialize = "dataSourceTitle"))]
    data_source: Option<String>,
    #[serde(rename(deserialize = "matchedName"))]
    matched_name: Option<String>,
    #[serde(rename(deserialize = "currentName"))]
    current_name: Option<String>,
    #[serde(rename(deserialize = "matchType"))]
    match_type: String,
    #[serde(rename(deserialize = "editDistance"))]
    edit_distance: Option<u8>,
}

fn main() {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.write_record(&[
        "page",
        "name",
        "odds",
        "match_type",
        "edit_distance",
        "matched_name",
        "source",
    ])
    .expect("Cannot write csv headers to stdout");
    for f in fs::read_dir("data").unwrap() {
        let path = f.unwrap().path();
        let file = path.file_stem().unwrap().to_str().unwrap();
        let page_num = get_page(file);
        let json = fs::read_to_string(&path).unwrap();
        let mut pg = serde_json::from_str::<Page>(json.as_str()).unwrap();
        pg.page = page_num;
        output(&mut wtr, pg).unwrap();
    }
}

fn output<W>(wtr: &mut csv::Writer<W>, page: Page) -> Result<(), Box<dyn Error>>
where
    W: io::Write,
{
    let page_id: String = format!("{}{}", BHL_URL, &page.page.to_string());
    let mut names = HashSet::<String>::new();
    if let Some(ns) = page.names {
        for n in ns {
            if names.contains(&n.name) {
                continue;
            }
            names.insert(n.name.clone());
            let odds = format!("{:.2}", n.odds);
            let br = n.verification.best_result;
            let matched_name = br.matched_name.unwrap_or("".to_owned());
            let mut edit_distance = br.edit_distance.unwrap_or(0).to_string();
            if br.match_type == "NoMatch" {
                edit_distance = "".to_owned();
            }
            let source = br.data_source.unwrap_or("".to_owned());
            wtr.write_record(&[
                &page_id,
                &n.name,
                &odds,
                &br.match_type,
                &edit_distance,
                &matched_name,
                &source,
            ])?;
        }
    }
    Ok(())
}

fn get_page(file: &str) -> u32 {
    let parts: Vec<_> = file.split("_").collect();
    if parts.len() > 1 {
        let num = parts.last().unwrap();
        if let Ok(n) = num.parse::<u32>() {
            return n;
        }
    }
    0
}
