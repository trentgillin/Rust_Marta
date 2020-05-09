#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_derive;
use structopt::StructOpt;
use reqwest::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = "Rusty Marta", about = "A small CLI to pull MARTA bus data")]
struct Opt {
    /// bus number
    #[structopt(short)]
    bus: String,
}

#[derive(Deserialize, Debug)]
struct Bus {
    TIMEPOINT: String,
    ADHERENCE: String,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let request_url = format!(
        "http://developer.itsmarta.com/BRDRestService/RestBusRealTimeService/GetBusByRoute/{bus_number}",
        bus_number = opt
    );
    let mut response = reqwest::get(request_url)?;

    let bus: Vec<Bus> = response.json()?;
    println!("{:?}", bus);
    Ok(())
}
