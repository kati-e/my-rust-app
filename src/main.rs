use structopt::StructOpt;
use exitfailure::ExitFailure;
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

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
        let api_key = "8fd6c3c1b57ee880648cf831a6b77ae4";
        let url = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}", city, country_code, api_key);
        let url = Url::parse(&url)?;

        let response = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;

    println!("our city: {}, our country code: {}", args.city, args.country_code);
    println!("Weather: {:?}", response.weather);

    println!(r"  |---------------------|");
    println!(r"  |  WELCOME TO MY APP  |");
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

    Ok(())
}
