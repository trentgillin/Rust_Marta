extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate structopt;
use reqwest::Error;
use serde::Deserialize;
use structopt::StructOpt;
use std::fmt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Rusty Marta", about = "A small CLI to pull MARTA bus data")]
struct Opt {
    // bus number
    #[structopt(short)]
    bus: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct Bus {
    timepoint: String,
    adherence: String,
    vehicle: String,
}

impl fmt::Display for Bus{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Station: {} | Time Away: {} | Bus ID: {}", self.timepoint, self.adherence, self.vehicle)
    }
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

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
        println!("There are {} buses running", bus.len());
        println!("=========================");
        for i in bus {
            println!("{}", i); 
        }
    };
    Ok(())
}
