use std::fs::File;
use std::env;
use std::collections::HashMap;
use std::io::prelude::*; //implements read_to_string fxn
// use std::error::Error;
use csv::Reader;
use regex::Regex;
// use std::time::Duration;
// use std::thread;



fn main() -> std::io::Result<()> {

    //create dictionary to store the sentiment scores
    let mut sentiment_table : HashMap<String, f32> = HashMap::new();

    //fill dictionary with key, value pairs from socialsent.csv
    sentiment_table = create_sentiment_table(sentiment_table);

    /* 
    Read command line arguments
    Format is ./main <filename>
    where filename is the name of the file to read
    */
    let args: Vec<String> = env::args().collect();

    /* if no file is provided in args, use review.txt as default file
       otherwise use the file provided by user */
    let filename;
    if args.len() < 2 {
        filename = "review.txt"; //default
    }
    else {
        filename = &args[1]; //user's choice
    }

    //open the file to read and determine sentiment score
    //the file descriptor will close automatically once it goes out of scope
    let mut file = File::open(filename)?;

    //initialize variable to store file data
    let mut contents = String::new();

    //store file data into contents
    file.read_to_string(&mut contents)?;

    //variable to hold total sentiment score
    let mut accumulated_score = 0.0;


    //define regex to correctly identify words in the list
    let re = Regex::new(r"\W").unwrap(); 

    println!("[word: current_score, accumulated_score]");
    for word in re.split(&contents){
        if let Some(current_score ) = sentiment_table.get(word) {
            accumulated_score += current_score;
            println!("{}: {:.2} {:.2}", word, current_score, accumulated_score);
        }
    }

    println!("\n{} score: {:.2}", filename, accumulated_score);
    
    Ok(())
}

fn create_sentiment_table(mut sentiment_table :  HashMap<String, f32>) -> HashMap<String, f32> {
    let sentiment_file = "socialsent.csv";

    let result = Reader::from_path(sentiment_file);

    if result.is_err() {
        println!("Failed to read CSV. File path probabily does not exist.");
        std::process::exit(1); //if file doesn't work exit immediately
    }

    let mut my_reader = result.unwrap();

    for record in my_reader.records() {
        let word = record.unwrap();
        sentiment_table.insert(
            word.get(0).unwrap().to_string(), 
            word.get(1).unwrap().parse::<f32>()
                .expect("Failed to convert to float"));
    }

    return sentiment_table;
}