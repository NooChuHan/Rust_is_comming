/* 라이브러리 사용 */
use std::io;
use std::cmp::Ordering;
use rand::Rng;
 
fn main() {
    /*
    1. 변수
    - immutable: let
    - mutable: let mut

    - 상수: const (※ 반드시 타입이 명시되어야 함)
      (e.g const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;)

    - ::new: 빈 객체 생성 (e.g. String::new -> 빈 문자열 생성)
    */
    let mut guess = String::new();

    /*
    2. 출력
    - println!: 문자열 출력 
     */
    println!("Guess the number!");
    println!("Please input your guess.");

    // rand::thread_rng(): 난수 생성기
    // get_range(start..=end): 범위 내 임의의 숫자 생성
    let secret_number = rand::thread_rng().gen_range(1..=1000);

    /*
    3. 입출력
    - stdin: 사용자 입력 처리 인스턴스
    - read_line: 사용자 입력을 문자열에 추가
    
    - & : 참조자
    - &mut: 가변 참조자
     */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // 타입 캐스팅을 위한 shadowing (변수 재사용)
    // shadowing: let으로 변수를 다시 선언하면 해당 스코프에서 변수를 덮어쓰게 된다.
    let guess: u32 = guess.trim().parse().expect("숫자를 입력해주세요.");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("숫자가 작습니다."),
        Ordering::Greater => println!("숫자가 큽니다."),
        Ordering::Equal => println!("정답입니다."),

    }
}
