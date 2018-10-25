use std::time::Instant;
use std::collections::HashMap;

mod benchmarking;
mod ackermann_implementations;

fn main() {
    let sample_size = 1000;
    let m = 3;
    let n = 8;
    let printing = false;
    
    benchmarking::run_ack_function(ackermann_implementations::hyper_ack::hyper_ack, "hyper_ack", 4, 100);
}