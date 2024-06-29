/*
- 바이너리 크레이트 루트: src/main.rs
- 라이브러리 크레이트 루트: src/lib.rs

모듈 트리는 src/lib.rs 내에 정의되어야 한다.
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

*/

mod front_of_house {
    // pub 키워드로 모듈 공개
    pub mod hosting {
        // 모듈이 공개되었다고 내용물도 공개되지는 않으므로 각 함수에 공개 여부를 별개로 설정해야 한다.
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// use 키워드로 모듈 가져오기 (shortcut)

// - use가 사용된 스코프에서만 단축  경로가 만들어지기 때문에 스코프가 달라지면 접근할 수 없다.
// - use로 함수를 가져올 때는 '부모 모듈'까지만 가져온다. 
use crate::front_of_house::hosting;
// - 구조체나 열거형 등 아이템을 가져올 때에는 '전체 경로'를 작성한다.
use std::collections::HashMap;

// - 단, 동일한 이름의 아이템을 여러개 가져올 때는 부모 모듈로 구분한다. (예시: Result)
use std::fmt;
use std::io;
fn function1() -> fmt::Result {}
fn function2() -> io::Result<()> {}

// as 키워드로 별칭을 지을 수 있다.
use std::io::Result as IoResult;
fn function2() -> IoResult<()> {}

// pub use를 사용하면 다른 스코프에서도 해당 모듈을 참조할 수 있다. (use만 쓰면 다른 스코프에서 비공개 처리됨!)
pub use crate::front_of_house::hosting;

// 모듈 가져오기 (단 모듈이 pub으로 공개되어야 접근 가능)
pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();

    // use 사용
    hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super 키워드를 사용하면 부모 모듈에서 시작하는 상대 경로를 만들 수 있다.
        super::deliver_order();
    }

    fn cook_order() {}

    // 구조체는 필드마다 공개/비공개 여부를 설정할 수 있다. default: 비공개
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // enum은 전체 공개/비공개만 설정할 수 있다. default: 공개
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// 중첩 경로를 쓰면 동일 모듈 내 여러 아이템을 가져올 때 코드를 줄일 수 있다. (하위 경로가 동일한 use 구문에서 유용)
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// 글롭 연산자(*)를 붙이면 경로 안에 정의된 모든 공개 아이템을 가져올 수 있다.
use std::collections::*;

fn main() {}