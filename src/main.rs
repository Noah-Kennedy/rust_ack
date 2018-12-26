#![feature(test)]
extern crate test;

use test::Bencher;

fn ack_simple(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack_simple(m - 1, 1)
    } else {
        ack_simple(m - 1, ack_simple(m, n - 1))
    }
}

fn ack_iter(m: u64, n: u64) -> u64 {
    hyperoperate(m, 2, n + 3) - 3
}

fn ack_opt(m: u64, n: u64) -> u64 {
    hyp_2(m, n + 3) - 3
}

fn hyp_2(m: u64, b: u64) -> u64 {
    match m {
        0 => b + 1,
        1 => 2 + b,
        2 => b << 1,
        3 => 2 << (b - 1),
        _ => (0..b - 1).fold(2, |old, _| hyperoperate(m - 1, old, 2))
    }
}

fn hyperoperate(m: u64, a: u64, b: u64) -> u64 {
    match m {
        0 => b + 1,
        1 => a + b,
        2 => a * b,
        _ => (0..b - 1).fold(a, |old, _| hyperoperate(m - 1, old, a))
    }
}

fn hyperoperate_pure(m: u64, a: u64, b: u64) -> u64 {
    if m == 0 {
        b + 1
    } else {
        let mut acc = a;

        if m == 1 {
            for _ in 0..b {
                acc = hyperoperate(m - 1, acc, b + 1);
            }
        } else {
            for _ in 0..b - 1 {
                acc = hyperoperate(m - 1, acc, a);
            }
        }

        acc
    }
}

fn ack_hyp(m: u64, n: u64) -> u64 {
    hyperoperate_pure(m, 2, n + 3) - 3
}

fn bench_ack<F>(ack: F, bench: &mut Bencher) where F: Fn(u64, u64) -> u64 {
    bench.iter(|| run_ack(&ack));
}

//#[bench]
//fn bench_ack_simple(b: &mut Bencher) {
//    bench_ack(ack_simple, b);
//}

#[bench]
fn bench_ack_iter(b: &mut Bencher) {
    bench_ack(ack_iter, b);
}

#[bench]
fn bench_ack_hyp(b: &mut Bencher) {
    bench_ack(ack_hyp, b);
}

#[bench]
fn bench_ack_opt(b: &mut Bencher) {
    bench_ack(ack_opt, b);
}

fn run_ack<F>(ack: F) where F: Fn(u64, u64) -> u64 {
    //for m in 0..4 {
    //    for n in 0..16 {
    //        println!("({}, {}) -> {}", m, n, ack(m, n));
    //    }
    //}

    let m = 3;
    let n = 60;
    println!("({}, {}) -> {}", m, n, ack(m, n));
}

fn main() {
    run_ack(ack_opt);
}