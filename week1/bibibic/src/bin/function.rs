/* example: 숫자 타입 매개변수 x와 y를 받고 숫자 타입을 반환하는 함수  */ 
fn example(x: i32, y: i32) -> i32 {
    x + y
}

/*

표현식

표현식 끝에 세미콜론을 추가하면 표현식은 구문으로 변경되어 값을 변환하지 않게 된다.
(구문은 값을 반환함)

반환값을 갖는 함수는 표현식과 마찬가지로 return 값에 세미콜론을 붙이지 않는다.

*/
fn expression() {
    let y = {
        let x = 3;
        x + 1 // 표현식은 세미콜론 X (and 값 반환)
    };

    println!("The value of y is: {y}");
}

fn main() {
    println!("example func value: {}", example(1, 2));
    expression();
}
