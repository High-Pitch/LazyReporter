use math::round;
use serde_json::{Result, Value};
use std::process::Command;

/*#[derive(Debug)] struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    user_id: i32,
}*/

struct Weather {
    //structure to make pulling json values easier
    json: String,
    temp: f64,
    prec: f64,
    high_of: f64
}
impl Weather {
    fn recommender(&self) -> (String, String) {
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
    fn process_data(&mut self) -> Result<()>{
            //Parsing string and converting to JSON file
            let data: Value = serde_json::from_str(&self.json)?;
            //Other variables for establishing and returning
            let mut inner_temp: f64 = 0.0;
            let mut inner_prec: f64 = 0.0;
            let amount_to_average = 6;
            //Grab the next 24 hours of temps and precepitation and average it
            for i in 0..amount_to_average {
                inner_temp += data["list"][i]["main"]["temp"].as_f64().expect("Temp not a number");
                inner_prec += data["list"][i]["pop"].as_f64().expect("Pop not a number");
            }
            let high_of = data["list"][0]["main"]["temp_max"].as_f64().expect("High of is not a number or does not exist"); 
            self.temp = inner_temp / amount_to_average as f64;
            self.prec = inner_prec / amount_to_average as f64;
            self.high_of = high_of;
            //let return_vec: Result<Vec<T>> = vec![temp, pop, high_of];
            Ok(())
        }
}

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

fn main() {
    use std::fs;
    let mut today = Weather { 
        json: fs::read_to_string("/home/engi/Documents/projects/weather_reporter/2024-10-02WeatherReport.json")
        .expect("Should have been able to read the file"),
        high_of: 0.0,
        temp: 0.0,
        prec: 0.0,
    };
    today.process_data().expect("Failure to process data");
    println!("{:?}", today.recommender());
}