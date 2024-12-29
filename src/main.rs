// src/main.rs

use std::fs::File;
use serde_json::Value;


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let f: File = File::open("meta-kaggle-code/0000/100/100295.ipynb")?;
    // IPYNB file is a JSON file. We can use serde_json to parse it.
    let reader = std::io::BufReader::new(f);
    let v: Value = serde_json::from_reader(reader)?;
    // println!("{:?}", v);

    if let Some(worksheets) = v["worksheets"].as_array() {
        for w in worksheets {
            if let Some(cells) = w["cells"].as_array() {
                for c in cells {
                    if let Some(input) = c["input"].as_str() {
                        println!("Cell input: {}", input);
                    }
                }
            }
        }
    }

    Ok(())
}