pub fn hyperoperate(m: u64, a: u64, b: u64) -> u64 {
    match m {
        0 => a + 1,
        1 => a + b,
        2 => a * b,
        _ => {
            let mut accumalator = 1;

            for i in 0..b {
                accumalator = hyperoperate(m - 1, accumalator, a);
            }

            if m == 1 {
                accumalator + 1
            } else {
                accumalator
            }
        }
    }
}

pub fn hyper_ack(m: u64, n: u64) -> u64 {
    hyperoperate(m, 2, n + 3) - 3
}

#[cfg(test)]
mod hyperoperator_tests {
    use super::*;

    #[test]
    fn test_incrementation() {
        assert_eq!(1, hyperoperate(0, 0, 0));

        assert_eq!(2, hyperoperate(0, 1, 0));
        assert_eq!(3, hyperoperate(0, 2, 0));
    }

    #[test]
    fn test_addition() {
        assert_eq!(0, hyperoperate(1, 0, 0));
        assert_eq!(1, hyperoperate(1, 0, 1));
        assert_eq!(1, hyperoperate(1, 1, 0));
        assert_eq!(5, hyperoperate(1, 3, 2));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(0, hyperoperate(2, 0, 0));
        assert_eq!(0, hyperoperate(2, 10, 0));
        assert_eq!(0, hyperoperate(2, 0, 10));

        assert_eq!(4, hyperoperate(2, 1, 4));
        assert_eq!(4, hyperoperate(2, 4, 1));

        assert_eq!(20, hyperoperate(2, 4, 5));
        assert_eq!(4, hyperoperate(2, 2, 2));
    }

    #[test]
    fn test_exponentiation() {
        assert_eq!(1, hyperoperate(3, 4, 0));
        assert_eq!(0, hyperoperate(3, 0, 4));

        assert_eq!(4, hyperoperate(3, 4, 1));
        assert_eq!(1, hyperoperate(3, 1, 4));

        assert_eq!(8, hyperoperate(3, 2, 3));
    }
}