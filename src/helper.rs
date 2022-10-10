use std::str::FromStr;

use serenity::framework::standard::Args;

pub fn parse_arg<T: FromStr>(arg: &Args) -> T {
    match arg.parse::<T>() {
        Ok(t) => t,
        Err(_) => panic!("Error parsing argument, crashing."),
    }
}
