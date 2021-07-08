// easy difficulty problems

pub fn problem1() -> i64 {
    // problem 1: https://projecteuler.net/problem=1
    let mut total: i64 = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
    }

    total
}

pub mod problem2 {
    // problem 2: https://projecteuler.net/problem=2
    fn is_even(n: &usize) -> bool {
        n % 2 == 0
    }

    pub fn problem2() -> usize {
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
