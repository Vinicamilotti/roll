#![feature(iter_array_chunks)]
pub mod rollib;
use crate::rollib::{generate_roll_requests, print_results};
use dicerollerlib::roll_request;
use dicerollerlib::rolltypes::{RollRequest, RollResult};
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let requests: Vec<RollRequest> = generate_roll_requests(args);
    let results: Vec<RollResult> = roll_request(requests);
    print_results(results)
}
