struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // 가변성은 인스턴스 전체가 지니게 되며, 일부 필드만 가변으로 만들 수는 없다.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // '구조체 업데이트 문법'을 사용하면 기존과 동일한 필드를 재사용할 수 있다.
    // 필드를 재사용할 ..user1은 가장 끝에 적어야 한다.
    // '구조체 업데이트 문법'은 데이터를 이동시키기 때문에, Copy 트레이드를 구현하지 않은 데이터가 이동된 이후로는 user1을 쓸 수 없다.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

// 변수명과 구조체 필드명이 같을 때는 '필드 초기화 축약법'을 쓸 수 있다.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}