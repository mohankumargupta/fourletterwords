use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct FourLetters {
    name: String,
    clues: Vec<String>,
    #[serde(default)]
    score: Option<f32>
}

#[derive(Serialize, Deserialize, Debug)]
struct FourLetterWords {
  word: String,
  score: u64,
  tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FourLetterWordsResponse {
  word: String,
  score: u64,
  tags: Vec<String>,
}

fn main() ->  io::Result<()> {
    let file = File::open("out2.json")?;
    let reader = BufReader::new(file);
    let input: Vec<FourLetters> = serde_json::from_reader(reader).unwrap();
    let output: Vec<FourLetters> = input.into_iter().filter(|item: &FourLetters|  item.clues.len() != 0
    ).collect();
    
    let write_file = File::create("out3.json").unwrap();
    let mut writer = BufWriter::new(&write_file);
    writeln!(writer, "{}", serde_json::to_string_pretty(&output)?).unwrap();

    //println!("{:?}", output);
    Ok(())
}