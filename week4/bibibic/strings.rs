/*
- 문자열은 UTF-8로 인코딩 된다.
- 러스트에서 문자열은 인덱스로 접근할 수 없다. -> UTF-8 구현에 의해 언어별로 차지하는 바이트가 다르기 때문 (+ 문자열 내 유효 문자를 검증하기 때문에 O(1)동작을 보장할 수 없어서)
*/ 


/* 새로운 문자열 생성하기 */
let mut s = String::new();
// 초기값을 가진 경우
let s = "initial contents".to_string(); 


/* 문자열 업데이트 */
let mut s = String::from("foo");
// push_str: 매개변수의 소유권을 가지지 않고 기존 문자열에 문자열 슬라이스를 추가한다.
s.push_str("bar");

// push: 한 개의 글자를  매개변수로 받아서 String에 추가한다.
let mut s = String::from("lo");
s.push('l');


// + 연산자
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
// +를 사용할 때 호출되는 함수의 시그니처(fn add(self, s:&str) -> String)와 맞추기 위해 s1은 소유권을 잃고 s2는 참조자를 사용한다.
// (&String 인수는 &str로 강제될 수 있다.)
// s3는 s1의 소유권을 가져다 s2의 복사본을 추가한 결과물의 소유권을 가져간다.
let s3 = s1 + &s2;


//  format! 연산자
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
// format!은 매개변수의 소유권을 가져가지 않으며 String을 반환한다.
let s = format!("{s1}-{s2}-{s3}");


/* 문자열 슬라이스 */
let hello = "Здравствуйте";
// 괄호 안 범위는 문자열의 인덱스가 아니라 바이트의 범위를 나타낸다.
let s = &hello[0..4];


/* 문자열 반복하기 */
// 문자 접근
for c in "Зд".chars() {
    // chars()을 사용하면 char 타입 문자에 하나씩 접근할 수 있다.
    println!("{c}");
}
// 바이트 접근
for b in "Зд".bytes() {
    // bytes()를 사용하면 원시 바이트에 하나씩 접근할 수 있다.
    println!("{b}");
}

fn main() {}