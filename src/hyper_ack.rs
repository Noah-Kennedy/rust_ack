pub fn hyperoperate(m: u64, a: u64, b: u64) -> u64 {
    if m == 0 {
        b + 1
    } else if m == 1 {
        a + b
    } else if m == 2 {
        a * b
    } else {
        let mut accumalator = a;
        
        for i in 1..b {
            accumalator = hyperoperate(m - 1, a, accumalator);
        }
        
        if m == 1 {
            accumalator + 1
        } else {
            accumalator
        }
    }
}

pub fn hyper_ack(m: u64, n: u64) -> u64 {
    hyperoperate(m, 2, n + 3) - 3
}