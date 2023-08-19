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

// Function to display happy crab with weather forecast \(^O^)/
pub fn happy_weather_crab(description: &str) {
    println!(r"  |---------------------|");
    println!(r"  |{}|", format_response(description));
    println!(r"  |---------------------|");
    println!(r"  |                     |");
    println!(r" /\/\ (O)       (O)   /\/\ ");
    println!(r" \  /  \\ ______//    \  / ");
    println!(r" / /   /         \     \ \ ");
    println!(r" \ \  |///   V   ///   / / ");
    println!(r"  \ \ \    ___     \  / /  ");
    println!(r"   \ \/    ___      \/ /   ");
    println!(r"    |     _______     |    ");
    println!(r"    |________________/     ")
}

//Function for where there is an error occuring (>-_-)>
pub fn sad_crab(err_message: &str) {
    println!(r"  |---------------------|");
    println!(r"  |{}|", format_response(err_message));
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

// Function that calculates/adds spaces before and after the response so it matches with the crab art nicely (/^-^)/
pub fn format_response(description: &str) -> String {
    let length_of_flag: usize = 21;

    if description.len() < length_of_flag {
        let diff = length_of_flag - description.len();
        let indiv_spacer_length = diff / 2;
        let spacer: String = std::iter::repeat(' ').take(indiv_spacer_length).collect();

        let new_desc: String;

        if description.len() % 2 != 0 {
            new_desc = format!("{}{}{}", spacer, description, spacer);
        } else {
            new_desc = format!("{}{}{} ", spacer, description, spacer);
        }

        new_desc
    } else {
        "".to_string() //haven't yet figured out what to do if the response is longer than the crab's flag
    }
}

impl Forecast {
    async fn get(city: &String, country_code: &String) -> Result<Self, ExitFailure> {
        dotenv().ok(); // Load api key from .env

        let api_key: String= match env::var("API_KEY") {
            Ok(key) => key,
            Err(_) => {
                sad_crab("No API Key Detected");
                std::process::exit(1); // Exit the program with a non-zero exit code
            }
        };

        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}",
            city, country_code, api_key
        );
        let url = Url::parse(&url)?;

        let response = reqwest::get(url).await?.json::<Forecast>().await?;

        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;

    let description = response.weather.get(0).map(|weather| &weather.description);

    // if there is a description val avail
    match description {
        Some(description) => {
            happy_weather_crab(description);
        }

        //else
        None => {
            sad_crab("No Weather Data");
        }
    }

    Ok(())
}
