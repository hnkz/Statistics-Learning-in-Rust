extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

mod kmc;

#[derive(Debug,Deserialize)]
struct Record {
    driver_id: i64,
    distance_feature: f64,
    speeding_feature: f64,
}

fn run() -> Result<Vec<Record>, Box<Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .double_quote(false)
        .flexible(true)
        .from_reader(file);

    let mut res: Vec<Record> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        res.push(record);
    }
    Ok(res)
}

fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        Some(file_path) => Ok(file_path),
        None => Err(From::from("expected 1 argument, but got none")),
    }
}

fn main() {
    let datas = match run() {
        Ok(val) => {
            val
        },
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    let mut points: Vec<(f64, f64)> = Vec::new();
    for data in datas {
        let point = (data.distance_feature, data.speeding_feature);
        points.push(point);
    }

    let mut kmc = kmc::KMeans::new(4, 1000);
    kmc.set_data(&points);
    kmc.predict_f();
}
