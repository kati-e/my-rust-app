use dotenv::dotenv;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    weather: Vec<Weather>,
    name: String,
    id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    id: i32,
    main: String,
    description: String,
}

pub fn sad_crab(err_message: String) {
    println!(r"  |---------------------|");
    println!(r"  |{}|", err_message);
    println!(r"  |---------------------|");
    println!(r"  |                     |");
    println!(r" /\/\ (_)       (_)   /\/\ ");
    println!(r" \  /  \\ ______//    \  / ");
    println!(r" / /   /         \     \ \ ");
    println!(r" \ \  |      ^    \    / / ");
    println!(r"  \ \ \    ___     \  / /  ");
    println!(r"   \ \/    ___      \/ /   ");
    println!(r"    |     _______     |    ");
    println!(r"    |________________/     ");
}

impl Forecast {
    async fn get(city: &String, country_code: &String) -> Result<Self, ExitFailure> {
        dotenv().ok(); // Load api key from .env
        
        // let api_key = env::var("API_KEY").expect("API_KEY environment variable not set");

        let api_key = match env::var("API_KEY") {
            Ok(key) => key,
            Err(_) => {
                sad_crab("No API Key Detected".to_string());
                std::process::exit(1); // Exit the program with a non-zero exit code
            }
        };

        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}",
            city, country_code, api_key
        );
        let url = Url::parse(&url)?;

        let response = reqwest::get(url).await?.json::<Forecast>().await?;

        // Need to do some error handling
        // But works if you put in correct vals
        // Such as: city: Brisbane, country_code: AU

        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;

    let description = response.weather.get(0).map(|weather| &weather.description);

    let length_of_flag: usize = 21;
    let mut first_spacer: String = " ".to_string();
    let mut second_spacer: String = " ".to_string();

    // if there is a description val avail
    match description {
        Some(description) => {
            if description.len() < length_of_flag {
                let diff = length_of_flag - description.len();

                let indiv_spacer_length = diff / 2;
                first_spacer = std::iter::repeat(' ').take(indiv_spacer_length).collect();

                if description.len() % 2 != 0 {
                    //have equal spacing on both sides if it is an odd number
                    second_spacer = std::iter::repeat(' ').take(indiv_spacer_length).collect();
                } else {
                    //take off a space from the end if it is even ?? dunno how this works but it does
                    second_spacer = std::iter::repeat(' ')
                        .take(indiv_spacer_length + 1)
                        .collect();
                }
            } else {
                // handle situation where the description is longer than the flag :o
            }

            println!(r"  |---------------------|");
            println!(r"  |{}{}{}|", first_spacer, description, second_spacer);
            println!(r"  |---------------------|");
            println!(r"  |                     |");
            println!(r" /\/\ (O)       (O)   /\/\ ");
            println!(r" \  /  \\ ______//    \  / ");
            println!(r" / /   /         \     \ \ ");
            println!(r" \ \  |///   V   ///   / / ");
            println!(r"  \ \ \    ___     \  / /  ");
            println!(r"   \ \/    ___      \/ /   ");
            println!(r"    |     _______     |    ");
            println!(r"    |________________/     ");
            println!(
                "The weather in {}, {} is {}.",
                args.city, args.country_code, description
            )
        }

        //else
        None => {
            sad_crab("No Weather Data".to_string());
        }
    }

    Ok(())
}
