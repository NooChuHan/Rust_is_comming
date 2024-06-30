pub fn day1() {
    // input -> get digits -> filter first and last -> sum

    let input = include_str!("example.txt");

    let result: Vec<u32> = input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            match (digits.first(), digits.last()) {
                (Some(&first), Some(&last)) => first * 10 + last,
                _ => 0,
            }
        })
        .collect();

    let sum = result.iter().fold(0, |acc, &x| acc + x);

    println!("{}", sum);
}
