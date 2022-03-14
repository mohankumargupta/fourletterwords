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

fn main_004() ->  io::Result<()> {
    let file = File::open("out3.json")?;
    let reader = BufReader::new(file);
    let input: Vec<FourLetters> = serde_json::from_reader(reader).unwrap();
    let output: Vec<FourLetters> = input.into_iter().map(|mut item: FourLetters|  {
      item.clues.retain(|item: &String| !item.contains("_"));  
      return item;   
    }).collect();
    
    let write_file = File::create("out4.json").unwrap();
    let mut writer = BufWriter::new(&write_file);
    writeln!(writer, "{}", serde_json::to_string_pretty(&output)?).unwrap();

    //println!("{:?}", output);
    Ok(())
}