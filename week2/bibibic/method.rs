#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl은 구조체 타입과 연관된 블록을 만들 때 쓰인다. (연관 함수)
impl Rectangle {
    // 메서드의 첫 번째 매개변수는 항상 self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // 매개변수를 여러개 가진 메서드
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 메서드가 아닌(self를 첫 매개변수로 갖지 않는) 연관 함수는 구조체의 새 인스턴스를 반환하는 생성자로도 활용된다.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// 구조체는 여러개의 impl 블록을가질 수 있다.
impl Rectangle {
    // 하지만 함수명은 중복될 수 없다. (width라고 지으면 에러 발생)
    fn width2(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );


    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
}