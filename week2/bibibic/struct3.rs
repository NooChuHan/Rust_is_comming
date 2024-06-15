// 외부 속성(outer attribute)
// 디버그 포맷에 사용될 수 있도록 명시적 동의
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // println!(
    //     "The area of the rectangle is {:#?} square pixels.",
    //     area(&rect1)
    // );

    // 디버그 포맷을 사용하여 값을 출력하기 위한 매크로
    // 표현식 소유권을 가져와서 매크로를 호출한 파일 및 라인 번호를 결과값과 함께 호출하고 소유권을 다시 반환한다.
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}