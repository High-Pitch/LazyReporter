//use math::round;
use std::io::*;
//use serde_json::{Result, Value};
//use reqwest::Error;


/*#[derive(Debug)] struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    user_id: i32,
}*/

/* struct Weather {
    //structure to make pulling json values easier
    temp: f64,
    prec: i16,
    high_of: i16,
}
impl Weather {
    fn recommender(self) -> (String, String) {
        //previously a full independent function in python, now internal to the struct
        //Values for recommendation basis
        let prec_low: f64 = 0.4;
        let prec_high: f64 = 0.6;
        let temp_low: f64 = 7.0;
        let temp_mid: f64 = 15.0;
        let temp_high: f64 = 20.0; 
    
        //rounding and message return creation
        let temp: f64 = round::stochastic(self.temp, 1);
        let prec: f64 = round::stochastic(self.prec.into(), 1); 
        let mut temp_message = String::new();
        let mut prec_message = String::new();
        
        if temp < temp_low {
            temp_message.push_str("Cold! Temperature today is: ");
            temp_message.push_str(&self.temp.to_string());
            temp_message.push_str("C with a high of ");
            temp_message.push_str(&self.high_of.to_string());
            temp_message.push_str("C You should grab a cold weather jacket.");
        } else if temp_low < temp && temp < temp_mid {
            temp_message.push_str("Mild Low. Temperature today is: ");
            temp_message.push_str(&temp.to_string());
            temp_message.push_str("C with a high of ");
            temp_message.push_str(&self.high_of.to_string());
        } else if temp_mid < temp && temp < temp_high {
            temp_message.push_str("Mild High. Temperature today is: ");
            temp_message.push_str(&temp.to_string());
            temp_message.push_str("C with a high of ");
            temp_message.push_str(&self.high_of.to_string());
            temp_message.push_str("C It's warm enough for a light jacket.");
        } else {
            temp_message.push_str("Hot! Temperature today is: ");
            temp_message.push_str(&temp.to_string());
            temp_message.push_str("C with a high of ");
            temp_message.push_str(&self.high_of.to_string());
            temp_message.push_str("C It is too warm for a jacket!");
        }
        if prec < prec_low {
            prec_message.push_str("No chance of rain today: ");
            prec_message.push_str(&prec.to_string());
        } else if prec_low < prec && prec < prec_high {
            prec_message.push_str("Slight chance of rain: ");
            prec_message.push_str(&prec.to_string());
        } else {
            prec_message.push_str("Rain likely, grab rain jacket");
            prec_message.push_str(&prec.to_string());

        }


        return (temp_message, prec_message)
        }
    }
*/

/* async fn get_data() -> Result<(), Error> {
    let data = fs::read_to_string("/home/engi/Documents/projects/weather_Reporter/api.txt").expect("Unable to read file");
    //Make API call
    let response = reqwest::get(data).await?;

    //let data: Value = serde_json::from_str(response)
    //.expect("No JSON");
    let body = response.text().await?
    println!("{}", body);
    Ok(())
}*/

fn process_data(content: &str) -> Result<()>{
    //Used imports
    use serde_json::Value;
    //Parsing string and converting to JSON file
    let data: Value = serde_json::from_str(content)?;
    //Other variables for establishing
    let mut temp: f64 = 0.0;
    let mut pop: f64 = 0.0;
    let amount_to_average = 6;
    //Grab the next 24 hours of temps and precepitation and average it
    for i in 0..amount_to_average {
        temp += data["list"][i]["main"]["temp"].from_value(f64);
        pop += data["list"][i]["main"]["pop"].from_value(f64);
    }
    let high_of = data["list"][0]["main"]["temp_max"]; 
    println!("{}", high_of);
    //println!("{}", data["list"][0]);
    Ok(())
}

fn file_write(data: &str) -> Result<()> {
    use chrono::Utc;
    use std::fs::*;

    //File name creation
    let now = Utc::now();
    let name :String = now.date_naive().to_string() + "WeatherReport.json";

    //Add data to the file
    let file = File::create(name);
    file?.write_fmt(format_args!("{}", data)).expect("File already exists");
    Ok(())
} 

fn main() {
    use std::fs;
    let content: &str = &fs::read_to_string("/home/engi/Documents/projects/weather_reporter/2024-10-02WeatherReport.json")
    .expect("Should have been able to read the file");
    //println!("{}", content);
    let _ = file_write("test");
    let _ = process_data(content);
}