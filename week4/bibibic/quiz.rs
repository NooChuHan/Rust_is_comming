// https://adventofcode.com/2023/day/1
fn quiz1(input: Vec<String>) -> i32 {
    let mut tmp: Vec<i32> = vec![];

    for str in input {
        // 10진수로 변환할 수 있는 문자만 반환하여 벡터 생성
        let num_list: Vec<_> = str.chars().filter_map(|c| c.to_digit(10)).collect();
        
        if !num_list.is_empty() {
            tmp.push(format!("{}{}", num_list[0], num_list[num_list.len() - 1]).parse::<i32>().unwrap())
        }
    }

    // println!("{:?}", tmp);
    tmp.iter().sum()
}

// https://adventofcode.com/2023/day/2
#[derive(Debug)]
struct Cube{
    r: i32, 
    g: i32, 
    b: i32
}
use regex::Regex;
fn quiz2(input1: Vec<String>, input2: String) -> i32 {

    fn find_rgb(input: Vec<&str>) -> Cube {
        let mut rgb = Cube {r: 0, g: 0, b: 0};
        for s in input {
            let split_tmp: Vec<_> = s.split_whitespace().collect();
            if split_tmp[1] == "red" {
                rgb.r = split_tmp[0].parse().unwrap();
            } 
            if split_tmp[1] == "green" {
                rgb.g = split_tmp[0].parse().unwrap();
            }
            if split_tmp[1] == "blue" {
                rgb.b = split_tmp[0].parse().unwrap();
            }
        }
        rgb
    }

    fn find_total_cube(string: String) -> Cube {
        let tmp: Vec<&str> = string.split(",").collect();
        find_rgb(tmp)
    }

    let total_cube: Cube = find_total_cube(input2);
    let mut result = 0;

    for (index, game) in input1.iter().enumerate() {
        let re = Regex::new(r"Game \d+: ").unwrap();
        let replaced_str = re.replace_all(game, "");
        let tmp: Vec<&str> = replaced_str.split(';').collect();
        let mut is_success = true;

        for el in tmp {
            let split_tmp = el.split(",").collect();
            let cube = find_rgb(split_tmp);

            if total_cube.r < cube.r || total_cube.g < cube.g || total_cube.b < cube.b {
                is_success = false;
                break;
            }
        }
        if is_success {
            result += index + 1;
        };
    }
    result.try_into().unwrap_or(0)
}

fn main() {
    let q1 = quiz1(vec!["1abc2".to_string(), "pqr3stu8vwx".to_string(), "a1b2c3d4e5f".to_string(), "treb7uchet".to_string()]);
    println!("{}", q1);

    let q2 = quiz2(vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
    ], "12 red cubes, 13 green cubes, 14 blue cubes".to_string());
    println!("{}", q2);
}
