use math::round;
use serde_json::{Result, Value};
use std::process::Command;
use std::fs;
use chrono::*;
use users::get_current_username;

struct Weather {
    //structure to make pulling json values easier
    json: String,
    temp: f64,
    prec: f64,
    high_of: f64
}
impl Weather {
    fn recommender(&self) -> String {
        //previously a full independent function in python, now internal to the struct
        //Values for recommendation basis
        let rain_quota: f64 = 0.6;
        let temp_winter: f64 = 2.0;
        let temp_low: f64 = 7.0;
        let temp_mid: f64 = 15.0;
        let temp_high: f64 = 20.0; 
    
        //rounding and message return creation
        let temp: f64 = round::stochastic(self.temp, 1);
        let prec: f64 = round::stochastic(self.prec.into(), 1); 
        let mut message = String::new();
        
        if rain_quota < prec {
            message.push_str("Rain-");
        }
        if temp < temp_winter {
            message.push_str("Winter");
        } else if temp_winter < temp && temp < temp_low {
            message.push_str("Heavy");
        } else if temp_low < temp && temp < temp_mid {
            message.push_str("Light");
        } else if temp_mid < temp && temp < temp_high {
            message.push_str("Style");
        } else {
            message.push_str("None");
        }
        return message
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
            self.temp = round::stochastic(inner_temp / amount_to_average as f64, 2);
            self.prec = round::stochastic(inner_prec / amount_to_average as f64, 2) * 100.0;
            self.high_of = high_of;
            Ok(())
        }
}

fn main() {
    let mut date: String = Local::now().date_naive().to_string();
    let user = format!("/home/{:?}",get_current_username());
    date.push_str("-WeatherReport.json");
    println!("{}",date);
    let mut today = Weather { 
        json: fs::read_to_string(format!("{:?}/weather_data/",user).to_owned() + &date)
        .expect("Should have been able to read the file"),
        high_of: 0.0,
        temp: 0.0,
        prec: 0.0,  
    };
    today.process_data().expect("Failure to process data");
    let t_arg: String = "-t ".to_owned() + &today.temp.to_string();
    let ho_arg: String = "-i ".to_owned() + &today.high_of.to_string();
    let r_arg: String = "-r ".to_owned() + &today.prec.to_string();
    let j_arg: String = "-j ".to_owned() + &today.recommender();
    let _command = Command::new(format!("{:?}/.virtualenvs/pimoroni/bin/python",user)).arg("weather-reporter-ui.py").arg(t_arg).arg(ho_arg).arg(r_arg).arg(j_arg).spawn().expect("Failed to execute");
}
