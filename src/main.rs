// use std::fs;
// use std::env;
use std::collections::HashMap;
// use std::error::Error;
use csv::Reader;
use std::time::Duration;
use std::thread;


// fn create_sentiment_dict(dict : &HashMap<String, f32>, sentiment_file : File) -> &HashMap<String, f32> {

//     //open sentiment scores file
//     let mut file = File::open(sentiment_file)
//         .expect("Oh no! The file {} provided in arguments could not be found...\n");

//     //store contents of sentiment file
//     let mut contents = String::new();
//     file.read_to_string(&mut contents);

//     println!("Contents: {}", contents);
    
//     return dict;
// }

fn main() {

    /* 
    Read command line arguments
    Format is ./main <filename>
    where filename is the name of the file to read
    */

    //create hashmap to store the sentiment scores
    let mut sentiment_dict : HashMap<String, f32> = HashMap::new();

    let sentiment_file = "data/socialsent.csv";

    let result = Reader::from_path(sentiment_file);

    if result.is_err() {
        println!("Failed to read CSV. File path probabily does not exist.");
        std::process::exit(1); //if file doesn't work exit immediately
    }

    let mut my_reader = result.unwrap();

    for record in my_reader.records() {
        let word = record.unwrap();
        sentiment_dict.insert(
            word.get(0).unwrap().to_string(), 
            word.get(1).unwrap().parse::<f32>().expect("Failed to convert to float"));
    }
    println!("{:?}", sentiment_dict);

    
    //gather the command line arguments provided by user
    //let args: Vec<String> = env::args().collect();
    
    /* if no file is provided in args, use review.txt as default file
       otherwise use the file provided by user */
    // let filename;
    // if args.len() < 2 {
    //     filename = "data/review.txt";
    // }
    // else {
    //     filename = &args[1];
    // }


    
}
