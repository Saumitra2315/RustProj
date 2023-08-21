use clap::{Parser, command};
#[derive(Parser)]
#[command(name= "wet")]
#[command(about = "weather in your terminal")]
struct Args{
    #[arg(short, default_value_t = 0)]
    days:u8,

}
fn main() {
    println!("Hello, world!");
}
