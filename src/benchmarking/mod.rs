use std::time::Instant;

pub fn benchmark_ack(fun: fn(u64, u64) -> u64, name: &str, sample_size: u64, m: u64, n: u64,
                 printing: bool) {
    let mut total: f64 = 0.0;
    for i in 0..sample_size {
        if printing {
            print!("Trial {}/{}:\t", i, sample_size);
        }
        
        let stopwatch = Instant::now();
        
        let _ = fun(m, n);
        
        let recorded_time = stopwatch.elapsed();
        let seconds = recorded_time.as_secs();
        let nanos = recorded_time.subsec_nanos();
        let duration = (seconds as f64) + ((nanos as f64) / 1000000000.0);
        
        if printing {
            println!("{}.{} seconds", seconds, nanos);
        }
        
        total += duration;
    }
    
    let average: f64 = total / (sample_size as f64);
    
    println!("{}({}, {}):\nSample Size:\t{}\nAverage Time:\t{}", name, m, n, sample_size, average);
}

pub fn run_ack_function(fun: fn(u64, u64) -> u64, name: &str, upper_m: u64, upper_n: u64) {
    for m in 0..upper_m {
        for n in 0..upper_n {
            let stopwatch = Instant::now();
            
            let result = fun(m, n);
            let recorded_time = stopwatch.elapsed();
            
            let seconds = recorded_time.as_secs();
            let nanos = recorded_time.subsec_nanos();
            
            println!("{}({}, {}) -> {}", name, m, n, result);
            println!("time:\t{}.{} seconds", seconds, nanos);
            println!();
        }
    }
}