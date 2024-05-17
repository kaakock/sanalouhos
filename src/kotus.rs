use std::error::Error;

use csv::ReaderBuilder;

const WORDS_CSV: &str = include_str!("./resources/sanalista.csv");

pub fn read_csv() -> Result<Vec<String>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_reader(WORDS_CSV.as_bytes());
    let mut results: Vec<String> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let word_res = record.get(0);
        match word_res {
            Some(x) => {
                if x.chars().count() > 2 {
                    results.push(x.to_string())
                }
            }
            None => (),
        }
    }
    Ok(results)
}
