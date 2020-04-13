use regex::{Captures, Regex};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use zstd;
#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct Record {
    id: String,
    name: String,
    canonical: String,
    meta: String,
}

impl Record {
    fn normalize(&mut self) {
        self.name = fix_unicode(&self.name);
        self.meta = fix_unicode(&self.meta);
    }
}

fn fix_unicode(s: &str) -> String {
    lazy_static! {
        static ref UNI: Regex = Regex::new(r"&#x(?P<n>[A-F0-9]+);").unwrap();
        static ref TAGS: Regex = Regex::new(r"</?(em|sub|sup|strong|l|r)>").unwrap();
    }
    let res = UNI
        .replace_all(s, |caps: &Captures| {
            let i = u32::from_str_radix(&caps[1], 16).unwrap();
            char::try_from(i).unwrap().to_string()
        })
        .into_owned();
    TAGS.replace_all(&res, "").into_owned()
}

#[derive(Debug)]
enum Data {
    Standard(Record),
    Nonstandard(Record),
}

fn main() {
    uncompress();
    read();
}

fn uncompress() {
    let f = File::open("data/2006.01.06.TaxonomicData.csv.zst").unwrap();
    let w = File::create("data/sherborn.csv").unwrap();
    zstd::stream::copy_decode(f, w).unwrap();
}

fn read() {
    let mut data: Vec<Data> = Vec::new();
    let f = File::open("data/sherborn.csv").unwrap();
    let reader = BufReader::new(f);
    for l in reader.lines() {
        let line = match l {
            Ok(ll) => ll,
            Err(_) => "".to_owned(),
        };
        let str = String::from_utf8_lossy(line.as_bytes());
        let fields: Vec<&str> = str.split(",").collect();
        if let Some(rec) = process(fields) {
            data.push(rec);
        }
    }
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    for d in data {
        match d {
            // Data::Standard(res) => println!("{}|{}|{}", res.id, res.canonical, res.name),
            Data::Standard(res) => wtr
                .write_record(&[res.id, res.canonical, res.name])
                .unwrap(),
            _ => (),
        }
    }
}

fn process(rec: Vec<&str>) -> Option<Data> {
    if rec.len() < 4 {
        return None;
    }
    let mut meta = "".to_owned();
    if rec.len() > 4 {
        meta = rec[4..].join(",");
    }
    let mut res = Record {
        id: rec[0].to_owned(),
        name: rec[3].to_owned(),
        canonical: "".to_owned(),
        meta,
    };
    res.normalize();
    if res.name.len() < 3 {
        return None;
    }
    let data = classify(res);
    Some(data)
}

fn classify(mut rec: Record) -> Data {
    let name_words = rec.name.split_whitespace().collect::<Vec<_>>();
    if name_words.len() > 1
        && name_words[0].chars().next().unwrap().is_lowercase()
        && name_words[1].chars().next().unwrap().is_uppercase()
    {
        rec.canonical = format!("{} {}", name_words[1], name_words[0]);
        return Data::Standard(rec);
    }
    classify_uninomials(rec)
}

fn classify_uninomials(mut rec: Record) -> Data {
    let mut uni = "".to_owned();
    let name = rec.name.replace(
        |ch: char| ch == '*' || ch == '†' || ch == ']' || ch == '[',
        "",
    );
    if name.chars().next().unwrap().is_uppercase() {
        let words = name.split_whitespace().collect::<Vec<_>>();
        if words.len() < 1 || words.len() > 8 {
            return Data::Nonstandard(rec);
        }
        uni = words[0].to_owned();
        for w in words {
            if w.chars().next().unwrap().is_lowercase() {
                return Data::Nonstandard(rec);
            }
        }
    }
    rec.canonical = uni;
    if rec.canonical.len() == 0 {
        return Data::Nonstandard(rec);
    }
    return Data::Standard(rec);
}
