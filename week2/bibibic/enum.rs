enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    // 특정데이터 구성 요소를  가진 열거형
    V4(u8, u8, u8, u8),
    // String 타입의 값을 가진 열거형
    V6(String),
}

// null 처럼, 값의 존재와 부재를 표현할 수 있는 열거형 Option<T>
// Option<T> ==! T
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    // 열거형을 정의할 때의 식별자로 네임스페이스가 만들어지기 때문에 각 variant 앞에 이중콜론을 붙여야 한다.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}