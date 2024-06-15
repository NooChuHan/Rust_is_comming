// match: 일련의 패턴에 대해 어떤 값을 비교한 뒤 어떤 패턴에 매칭되었는지를 바탕으로 코드를 수행하도록 하는 제어 흐름 연산자
#[derive(Debug)]
enum UsState {
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
    // match 표현식은 어떤 타입이든 반환할 수 있다.
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } // 중괄호를 쓰면 쉼표는 붙여도 되고, 안 붙여도 된다.
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn dice_roll(dice: i32) {
     match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // default인 케이스를 처리하는 포괄 처리자
        // other => move_player(other),
        // 포괄 패턴이 필요하지만 사용할 필요가 없을 때는 other 대신 _를 쓴다.
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
