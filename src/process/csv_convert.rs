use std::fs;

use csv::Reader;

use crate::opts::OutputFormat;

// #[derive(Debug, serde::Deserialize, serde::Serialize)]
// struct Player {
//     #[serde(rename = "Name")]
//     name: String,
//     #[serde(rename = "Position")]
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     #[serde(rename = "Nationality")]
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit: u8,
// }

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret: Vec<serde_json::Value> = Vec::with_capacity(128);
    // let headers = reader.headers()?; // borrow mutable reference once
    let headers = reader.headers()?.clone();

    // let records = reader
    //     .deserialize()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<Player>>();
    for result in reader.records() {
        let record = result?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        // OutputFormat::Toml => toml::to_string_pretty(&ret)?,
    };

    fs::write(output, content)?; // => () unit
    Ok(())
}
