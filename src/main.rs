use std::fs::File; //crate for handling files
use std::env; //crate for handling arguments
use std::collections::HashMap; //crate to implement dictionary
use std::io::prelude::*; //implements read_to_string fxn
// use std::error::Error;
use csv::Reader; //crate to easily read CSV files
use regex::Regex; //crate to allow regex for easier string parsing

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

    let accumulated_score = get_sentiment_score(filename, sentiment_table);

    //determine star rating based on total sentiment value that was calculated
    let star_rating = get_star_rating(accumulated_score);

    //display final results of sentiment scoring calculation
    println!("\n{} score: {:.2}", filename, accumulated_score);
    println!("{} Stars: {}", filename, star_rating);

    Ok(()) //successful execution of program

}

fn get_star_rating(accumulated_score : f32) -> i32 {

    //determine star rating based on total sentiment value that was calculated
    let star_rating = match accumulated_score {

        n if n < -5.0 => 1,

        n if n >= -5.0 && n < -1.0 => 2,

        n if n >= -1.0 && n < 1.0 => 3,

        n if n >= 1.0 && n < 5.0 => 4,

        n if n >= 5.0 => 5,

        _ => 0,
    };

    return star_rating;
}

fn get_sentiment_score(filename : &str, sentiment_table : HashMap<String, f32>) -> f32 {
    //open the file to read and determine sentiment score
    //the file descriptor will close automatically once it goes out of scope
    let mut file = File::open(filename)
        .expect("Failed to open file.");

    //initialize variable to store file data
    let mut contents = String::new();

    //store file data into contents
    file.read_to_string(&mut contents)
        .expect("Failed to read file.");

    //variable to hold total sentiment score
    let mut accumulated_score = 0.0;


    //define regex to correctly split words on whitespace in the list
    let re = Regex::new(r"[\t \n\r\f]").unwrap(); 

    //print header for output
    println!("[word: current_score, accumulated_score]");

    //iterate through the words in the contents retrieved from file
    for word in re.split(&contents){

        //checks if the word is present in the sentiment table
        if let Some(current_score ) = sentiment_table.get(word) {
            accumulated_score += current_score; //add score of word to accumulated score

            //display the word, its score, and the accumulated score at that moment
            println!("{}: {:.2} {:.2}", word, current_score, accumulated_score);
        }
    }

    //return the total score for the text that was calculated
    return accumulated_score;
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