use std::time::Instant;

fn main() {
    let sample_size = 1000;
    let m = 3;
    let n = 9;
    
    run_ack_function(ack_if, "ack_if", 4, 100);
    //enchmark_ack(ack_if, "ack_if", sample_size, m, n, false);
    //enchmark_ack(ack_match_each, "ack_match_each", sample_size, m, n, false);
    //enchmark_ack(ack_match_together, "ack_match_together", sample_size, m, n, false);
    
}

fn benchmark_ack(fun: fn(u64, u64) -> u64, name: &str, sample_size: u64, m: u64, n: u64,
                 printing: bool) {
    let mut total: f64 = 0.0;
    for i in 0 .. sample_size {
        
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

fn run_ack_function(fun: fn(u64, u64) -> u64, name: &str, upper_m: u64, upper_n: u64) {
    for m in 0 .. upper_m {
        for n in 0 .. upper_n {
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

fn ack_if(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack_if(m - 1, 1)
    } else {
        ack_if(m - 1, ack_if(m, n - 1))
    }
}

fn ack_match_each(m: u64, n: u64) -> u64 {
    match m {
        0 => n + 1,
        _ => match n {
            0 => ack_match_each(m - 1, 1),
            _ => ack_match_each(m - 1, ack_match_each(m, n - 1))
        }
    }
}

fn ack_match_together(m: u64, n: u64) -> u64 {
    match (m, n) {
        (0, _) => n + 1,
        (_, 0) => ack_match_together(m - 1, 1),
        (_, _) => ack_match_together(m - 1, ack_match_together(m, n - 1))
    }
}