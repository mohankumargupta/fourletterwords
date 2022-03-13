use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use soup::prelude::*;
use serde::Serialize;
use regex::Regex;

#[derive(Serialize)]
struct FourLetters {
    name: String,
    clues: Vec<String>
}

fn 001_main() -> io::Result<()> {
    let file = File::open("4-letter-words.txt")?;
    let reader = BufReader::new(file);
 
    let mut fourlettersjson: Vec<FourLetters> = Vec::new();

    for line in reader.lines() {
        let result = line?;
        //println!("----------");
        //println!("{}", &result);
        let clues = find_clues(&result);
        let fourletters = FourLetters {name: result, clues: clues};
        fourlettersjson.push(fourletters);
        
        //println!("{}", serde_json::to_string_pretty(&fourletters)?);
        
    }
    //println!("{}", serde_json::to_string_pretty(&fourlettersjson)?);
    let write_file = File::create("out.json").unwrap();
    let mut writer = BufWriter::new(&write_file);
    writeln!(writer, "{}", serde_json::to_string_pretty(&fourlettersjson)?).unwrap();
    Ok(())
}

fn find_clues(line: &String) -> Vec<String> {
    let url = format!("http://www.the-crossword-solver.com/word/{line}/") ;
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).header("User-Agent","chrome").send().unwrap();
    let soup = Soup::from_reader(response).unwrap();
    let mut result = Vec::<String>::new();
    soup.tag("td").find().and_then(|td|{
     let b = td.tag("a").limit(6).find_all().collect::<Vec<_>>();  
     let mut j = 0;
     //println!("{}", &line);
     for i in &b {
        if j!=1 {
        //println!("{}", i.text());
        let re = Regex::new(r"([^(]+)\s+\([^)]+\)").unwrap();
        let boo = &i.text();
        println!("----{}----", &line);
        println!("{}", boo);
        if let Some(capture) = re.captures(boo) {
            result.push(capture.get(1).unwrap().as_str().to_string())
        }
        //println!("{}", capture.get(1).unwrap().as_str());
        //result.push(i.text().clone());
        
        }
        j = j + 1;
     } 
     
     td.tag("a").find()
    });
    
    //println!("{}", &result[1]);
    //result.remove(1);
    result
    //result.sort_by(|a,b| a.len().cmp(&b.len()));
    //println!("{}", result[0]);

}



