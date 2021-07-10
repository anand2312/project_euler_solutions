// easy difficulty problems

pub mod problem1 {
    pub fn solution() -> i64 {
        // problem 1: https://projecteuler.net/problem=1
        let mut total: i64 = 0;

        for i in 1..1000 {
            if i % 3 == 0 || i % 5 == 0 {
                total += i;
            }
        }

        total
    }
}

pub mod problem2 {
    // problem 2: https://projecteuler.net/problem=2
    fn is_even(n: &usize) -> bool {
        n % 2 == 0
    }

    pub fn solution() -> usize {
        let mut v = vec![1, 2];
        let mut sum = 2;

        loop {
            let to_push = v[v.len() - 1] + v[v.len() - 2];

            if to_push > 4000000 {
                break
            }

            if is_even(&to_push) {
                sum += to_push
            }
            v.push(to_push);
        }

        sum
    }
}


pub mod problem3 {
    use std::usize;

    // problem 3: https://projecteuler.net/problem=3
    pub fn solution() {
        let mut num: f64 = 600851475143;
        let mut primes: Vec<_> = vec![];

        while  i % 2 == 0 {
            primes.push(2);
            num = num / 2;
        }

        let sqrt = num.sqrt() as usize + 1;

        for i in (3..sqrt).step_by(2) {
            while n % i == 0 {
                primes.push(i);
                num = num / i as f64;
            }
        }

        if num > 2.0 {
            // number itself is prime
            primes.push(num as usize);
        }


    }

    fn max(arr: Vec<usize>) -> usize {
        let mut biggest: usize = 0;
        for i in arr.iter() {
            if i > biggest {
                biggest = i;
            }
        }
        biggest
    }
}