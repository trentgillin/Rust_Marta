extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate structopt;
use reqwest::Error;
use serde::Deserialize;
use std::env;
use std::fmt;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Rusty Marta", about = "A small CLI to pull MARTA data")]
struct Opt {
    // bus number
    #[structopt(short, default_value = "No Bus Selected")]
    bus: String,

    // train route
    #[structopt(short, default_value = "Not Selected")]
    train: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct Bus {
    timepoint: String,
    adherence: String,
    vehicle: String,
}

impl fmt::Display for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Station: {} | Time Away: {} | Bus ID: {}",
            self.timepoint, self.adherence, self.vehicle
        )
    }
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    if opt.bus != "No Bus Selected" {
        let request_url = format!(
            "http://developer.itsmarta.com/BRDRestService/RestBusRealTimeService/GetBusByRoute/{}",
            opt.bus
        );

        let response = reqwest::blocking::get(&request_url)?;
        let bus: Vec<Bus> = response.json()?;

        // handle buses with no times currently
        if bus.len() == 0 {
            println!("No times for bus: {}", opt.bus)
        } else {
            println!("There are {} bus(es) running", bus.len());
            println!("=========================");
            for i in bus {
                println!("{}", i);
            }
        };
    } else {
        // Does the user have a Marta API Token stored in the environment variables
        let test_token = env::var("MARTA_TOKEN").is_err();

        let test_token = if test_token {
            println!("You Need a MARTA API Token, you can get one at https://www.itsmarta.com/developer-reg-rtt.aspx");
            println!("Please enter your API Token or hit Ctrl+C to end the program:");

            // Get Users token
            let mut token = String::new();

            io::stdin()
                .read_line(&mut token)
                .expect("Failed to read line");

            // set to environment variable
            let marta_token = "MARTA_TOKEN";
            env::set_var(marta_token, &token);
        };

        let marta_token = env::var("MARTA_TOKEN");
        println!("{:?}", marta_token);
    };
    Ok(())
}
