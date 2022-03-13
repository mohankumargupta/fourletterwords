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

fn get_response(url: String) -> f32 {
    println!("{}", url);
    let res: reqwest::Result<reqwest::blocking::Response> = reqwest::blocking::get(url);
    let resp = res.unwrap().json::<Vec<FourLetterWords>>().unwrap();
    //println!("{:#?}", resp[0].tags);
    let needle = "f:".to_string();
    let haystack = &resp[0].tags;
  
    if let Some(word) = haystack.into_iter().find(|&s| s.starts_with(&needle)) {
      let score = &word[2..];
      let numeric_score: f32 = score.parse().unwrap();
      return numeric_score;
    }
  
    return 0.0;
  }

fn main() ->  io::Result<()> {
    let file = File::open("out.json")?;
    let reader = BufReader::new(file);
    let input: Vec<FourLetters> = serde_json::from_reader(reader).unwrap();
    let output: Vec<FourLetters> = input.into_iter().map(|mut item: FourLetters|  {
        let url = format!("https://api.datamuse.com/words?sp={}&qe=sp&md=f&max=1", item.name);
        let score = get_response(url);
        item.score = Some(score);
        return item;
    }).collect();
    
    println!("{:?}", output);
    Ok(())
}