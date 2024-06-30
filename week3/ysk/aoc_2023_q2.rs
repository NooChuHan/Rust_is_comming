fn parse_game(line: &str) -> (u32, Vec<(u32, u32, u32)>) {
    let parts: Vec<&str> = line.split(":").collect();
    let game_id = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();

    let sets = parts[1]
        .split(";")
        .map(|set| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cube in set.split(",") {
                let cube_parts: Vec<&str> = cube.trim().split_whitespace().collect();
                let count: u32 = cube_parts[0].parse().unwrap();
                match cube_parts[1] {
                    "red" => red = count,
                    "green" => green = count,
                    "blue" => blue = count,
                    _ => {}
                }
            }

            (red, green, blue)
        })
        .collect();

    (game_id, sets)
}

pub fn day2() {
    // input -> parse_game -> valid_games -> sum

    let input = include_str!("example.txt");

    let games: Vec<(u32, Vec<(u32, u32, u32)>)> = input.lines().map(parse_game).collect();

    let possible_games: Vec<u32> = games
        .iter()
        .filter(|(_, sets)| sets.iter().all(|&(r, g, b)| r <= 12 && g <= 13 && b <= 14))
        .map(|&(id, _)| id)
        .collect();

    let sum: u32 = possible_games.iter().sum();

    println!("Sum of IDs of possible games: {}", sum);
}
