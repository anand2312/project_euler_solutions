// easy difficulty problems

pub fn problem1() {
    // problem 1: // problem 1: https://projecteuler.net/problem=1
    let mut total: i64 = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
    }

    println!("Sum of numbers who are multiples of 3 or 5: {}", total)
}
