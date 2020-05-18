#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_derive;
extern crate structopt;
use reqwest::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Rusty Marta", about = "A small CLI to pull MARTA bus data")]
struct Opt {
    // bus number
    #[structopt(short)]
    bus: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct Bus {
    timepoint: String,
    adherence: String,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let request_url = format!(
        "http://developer.itsmarta.com/BRDRestService/RestBusRealTimeService/GetBusByRoute/{}",
        opt.bus
    );
    let mut response = reqwest::get(&request_url)?;
    let bus: Vec<Bus> = response.json()?;

    // handle buses with no times currently
    if bus.len() == 0 {
        println!("No times for bus: {}", opt.bus)
    } else {
        println!("{:?}", bus);
    };
    Ok(())
}
