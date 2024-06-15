
fn main() {
    let config_max = Some(3u8);

    // if let은 하나의 패턴만 매칭시켜야 하는 경우에 사용한다.
    // 값이  Some이면 max에 Some 배리언트의 값을 바인딩하고 출력
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // if-else 케이스
    // let mut count = 0;

    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // if let-else 케이스
    // let mut count = 0;
    
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }
}