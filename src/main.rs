use structopt::StructOpt;
use exitfailure::ExitFailure;
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;
use dotenv::dotenv;
use std::env;

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

impl Forecast {
    async fn get(city: &String, country_code: &String) -> Result<Self, ExitFailure> {
        dotenv().ok(); // Load api key from .env
        let api_key = env::var("API_KEY").expect("API_KEY environment variable not set");
        let url = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}", city, country_code, api_key);
        let url = Url::parse(&url)?;

        let response = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
            
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

    // if there is a description val avail
    match description {
        Some(description) => {
        println!(r"  |---------------------|");
        println!(r"      {}   ", description);
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
        println!("The weather in {}, {} is {}.", args.city, args.country_code, description)},
        //else
        None => {
            sad_crab();
        },
    }

    fn sad_crab() {
        println!(r"  |---------------------|");
        println!(r"  |  No Data Available  |");
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

    Ok(())
}
