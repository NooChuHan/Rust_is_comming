// 구조체에는 이름이 있지만 필드명이 없는 '튜플 구조체'
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 필드가 아예 없는 '유사 유닛 ㄱ ㅜ조체'
struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}