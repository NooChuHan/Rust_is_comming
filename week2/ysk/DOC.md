## 백터

백터는 표준 라이브러리에서 제공하는 컬렉션 중 하나이다. 백터의 경우 가변의 데이터를 저장하기 위해 유용한 컬렉션이다.

```rust
// 선언
let v: Vec<i32> = Vec::new();

// initial value와 선언
let v = vec![1, 2, 3];
```

- **순회**
    
    ```rust
    // 일반 순회
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    
    // 가변참조 순회
    let mut v = vec![100, 32, 57];
    for i in &mut v // 가변참조자 {
        *i += 50; // 역참조 
    }
    
    for element in v.iter_mut() {
      *element *= 2;
    }
    ```
    
- **배열 vs 백터**
    - 배열
        - **고정 크기**: 배열의 크기는 고정되어 있으며, 생성 시에 크기를 명시해야 함.
        - **컴파일 타임 크기**: 배열의 크기는 컴파일 타임에 결정됨.
        - **메모리 할당**: 배열은 스택에 할당.
        - **문법**: `[T; N]` 형식으로 정의됩니다. 여기서 `T`는 요소의 타입, `N`은 배열의 크기.
    - 백터
        - **가변 크기**: 벡터는 동적 배열로, 런타임에 크기가 변경될 수 있다.
        - **동적 크기**: 벡터의 크기는 런타임에 결정되고 필요에 따라 커질 수 있다.
        - **메모리 할당**: 벡터는 힙에 할당된다.
        - **문법**: `Vec<T>` 형식으로 정의됩니다. 여기서 `T`는 요소의 타입.
        - **유연성**: 벡터는 크기를 동적으로 조절할 수 있어 유연성이 높다.

## 소유권

소유권은 러스트 프로그램의 메모리 관리법을 지배하는 규칙이다.

- 스택
    - 후입선출
    - 각 스택에 저장된 데이터의 크기는 동일하다.
    - 값
        - 모든 정수형 타입 (예: `u32`)
        - `true`, `false` 값을 갖는 논리 자료형 `bool`
        - 모든 부동 소수점 타입 (예: `f64`)
        - 문자 타입 `char`
        - `Copy` 가능한 타입만으로 구성된 튜플 (예를 들어, `(i32, i32)`는 `Copy` 가능하지만 `(i32, String)`은 불가능합니다)
- 힙
    - 힙에 넣을 때 저장할 공간이 있는지 OS에 확인 하고 그 빈 지점에 데이터를 넣는다. → 이후 포인터를 반환
    - 값
        - 벡터
        - String
        - 등등
- 규칙
    - 러스트에서, 각각의 값은 *소유자 (owner)* 가 정해져 있습니다.
    - 한 값의 소유자는 동시에 여럿 존재할 수 없습니다.
    - 소유자가 스코프 밖으로 벗어날 때, 값은 버려집니다 (dropped). → GC에서 하는 것이 아님
- 소유권 이동
    
    ```rust
    let vec0 = vec![22, 44, 66];
    
    let vec1 = vec0; // vec0의 소유권은 vec1으로 넘어감.
    
    println!(vec0); // borrow of moved value: `vec0`
    ```
    
    - 힙에 저장되는 값의 경우 값에 할당 하고자 했을 때, 스택에 저장 되는 데이터와는 달리 포인터를 반환하기에, ownership transfer가 된다.
    
    ```rust
    fn main() {
        let vec0 = vec![22, 44, 66];
    
        let vec1 = fill_vec(vec0); // 이렇게 param으로 넘기는 것 만으로도 소유권 이동이 된다.
    
        assert_eq!(vec0, vec![22, 44, 66]); // ERROR!! 소유권을 이미 잃어버렸기에 다시는 사용할 수 없음
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
    
    fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
        let mut vec = vec;
    
        vec.push(88);
    
        vec // 만약, vec를 return 하지 않았다면, 소유권 이동된 vec0의 값의 생명주기는 해당 함수에서 끝났을 것.
    }
    
    ```
    
- 값의 복사
    
    ```rust
    fn main() {
        let vec0 = vec![22, 44, 66];
    
        let vec1 = fill_vec(vec0.clone());
    
        assert_eq!(vec0, vec![22, 44, 66]);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
    
    fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
        let mut vec = vec;
    
        vec.push(88);
    
        vec
    }
    ```
    
    - 힙 데이터의 경우 깊은 복사를 위한 `clone` 이라는 공용 메서드가 있다.
- 참조와 대여
    
    ```rust
    
    fn main() {
        let data = "Rust is great!".to_string();
    
        get_char(&data); // 참조를 넘김
    
        string_uppercase(data); // 소유권을 넘김
    }
    
    // Should not take ownership
    fn get_char(data: &String) -> char { // 참조자 data를 받겠다.
        data.chars().last().unwrap()
    }
    
    // Should take ownership
    fn string_uppercase(mut data: String) { // 소유권을 이동 받겠다.
        data = data.to_uppercase();
    
        println!("{}", data);
    }
    ```
    
    - 참조자 라는 것은 해당 주소에 저장된 데이터에 접근할 수 있도록 해주는 주솟값에 해당하는, 포인터이다.
    - 참조자를 넘기게 되면, 소유권은 이동하지 않는다.
    - 이러한 행위를 “대여”라고 한다.
- 가변 참조자
    
    ```rust
    fn main() {
        let s = String::from("hello");
    
        change(&s);
    }
    
    fn change(some_string: &String) {
        some_string.push_str(", world"); // 대여된 기본 참조자는 수정하지 못한다.
    }
    
    /* ................................. */
    
    fn main() {
        let mut s = String::from("hello");
    
        change(&mut s);
    }
    
    fn change(some_string: &mut String) {
        some_string.push_str(", world"); // 가변 참조자를 대여 받는다면, 수정할 수 있다.
    }
    ```
    
    - 기본 참조자를 대여 받는 경우 수정하지 못하나, `mut` 키워드가 붙은 가변 참조자를 사용하면 가능하다.
    - 따라서, 가변 참조자의 경우 `&mut` 으로 명시해야 한다.
    
    ```rust
    fn main() {
        let mut x = 100;
        let y = &mut x;  // 첫 번째 가변 참조
        let z = &mut x;  // 두 번째 가변 참조 - 오류 발생
        *y += 100;
        *z += 1000;
        assert_eq!(x, 1200);
    }
    ```
    
    - 하지만, 같은 참조자를 여러번 동시에 가변 참조자로 만들어서 사용할 수 없다.
    - 데이터 경합 조건
        - 둘 이상의 포인터가 동시에 같은 데이터에 접근
        - 포인터 중 하나 이상이 데이터에 쓰기 작업을 시행
        - 데이터 접근 동기화 메커니즘이 없음
    - 참조자 규칙
        - 단 하나의 가변 참조자만 갖거나, 여러 개의 불변 참조자를 가질 수 있다.
        - 참조자는 항상 유효해야 한다.
- 역참조
    
    ```rust
    fn main() {
        let x = 10;
        let y = &x;  // y는 x에 대한 불변 참조
        println!("x의 값: {}", x);  // x의 값: 10
        println!("y가 참조하는 값: {}", *y);  // y가 참조하는 값: 10
    
        let mut a = 20;
        let b = &mut a;  // b는 a에 대한 가변 참조
        *b += 5;  // b를 통해 a의 값을 수정
        println!("a의 값: {}", a);  // a의 값: 25
    }
    ```
    
    - 역참조(dereference)는 참조(reference)된 값을 직접 접근하는 것을 의미한다.
    - 참조는 포인터와 유사하여 메모리 주소를 가리킨다. 따라서 참조를 통해 직접적인 값에 접근하려면 Asterisk(*) 연산자를 사용하여 역참조해야 한다.
- 정리
    - **참조**: 변수의 메모리 주소를 가리키는 포인터.
    - **역참조**: 참조된 값을 직접 접근하는 연산으로, Asterisk 연산자를 사용합니다.
    - **불변 참조**: 값을 읽을 수 있지만 수정할 수 없습니다.
    - **가변 참조**: 값을 읽고 수정할 수 있습니다.

## 구조체

구조체는 사용자 계층 정보를 저장하는 데이터 구조이며, 여러 필드에 다양한 타입을 담을 수 있는 자료 구조이다. ( 단, 함수의 경우 `impl` 로 정의해야 한다. )

- 구조체 사용
    
    ```rust
    struct ColorClassicStruct {
        red: u8,
        green: u8,
        blue: u8,
    }
    
    ...
    
    let green = ColorClassicStruct {
        red: 0,
        green: 255,
        blue: 0,
    };
    
    // 튜플 구조체
    struct ColorTupleStruct(
        u8,
        u8,
        u8,
    );
    
    let green = ColorTupleStruct(0, 255, 0);
    
    // unit 구조체
    let unit_like_struct = UnitLikeStruct;
    ```
    
- 필드 spead
    
    ```rust
    struct UnitLikeStruct;
    
    ...
    
    let your_order = Order {
        name: String::from("Hacker in Rust"),
        count: 1,
        // 이렇게 스프래드로 템플릿에 대해 넣어줄 수 있다.
        ..order_template
    };
    ```
    
    - js랑 같으나, 주의할 점은 `..` 임.
- 연관 함수 impl ( implementation )
    
    ```rust
    struct Package {
        sender_country: String,
        recipient_country: String,
        weight_in_grams: u32,
    }
    
    // impl 구현을 통해 메소드를 구현할 수 있다.
    impl Package {
        fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
            if weight_in_grams < 10 {
                // This is not how you should handle errors in Rust,
                // but we will learn about error handling later.
                panic!("Can not ship a package with weight below 10 grams.")
            } else {
                Package {
                    sender_country,
                    recipient_country,
                    weight_in_grams,
                }
            }
        }
    
        fn is_international(&self) -> bool {
            self.sender_country != self.recipient_country
        }
    
        fn get_fees(&self, cents_per_gram: u32) -> u32 {
            self.weight_in_grams * cents_per_gram
        }
    }
    
    let package = Package::new(sender_country, recipient_country, 1200);
    
    assert!(!package.is_international());
    ```
    
    - 정의 하고자 하는 구조체의 이름과 동일해야 한다. ( for로 특정 구조체를 지정할 수 있음 )
    - 참고로 여러 개의 impl 블록을 가질 수 있다.
- trait
    
    ```rust
    // 트레이트 정의
    trait Greet {
        fn greet(&self);
    
        // 기본 메서드 구현
        fn greet_with_default(&self) {
            println!("Hello, this is the default greeting!");
        }
    }
    
    // 구조체 정의
    struct Person {
        name: String,
    }
    
    struct Dog {
        name: String,
    }
    
    // 트레이트 구현: Person 구조체에 대한 Greet 트레이트 구현
    impl Greet for Person {
        fn greet(&self) {
            println!("Hello, my name is {}.", self.name);
        }
    }
    
    // 트레이트 구현: Dog 구조체에 대한 Greet 트레이트 구현
    impl Greet for Dog {
        fn greet(&self) {
            println!("Woof! My name is {}.", self.name);
        }
    }
    
    // 제네릭 함수: Greet 트레이트를 구현한 타입만 허용
    fn print_greeting<T: Greet>(item: T) {
        item.greet();
        item.greet_with_default();  // 기본 메서드 호출
    }
    
    fn main() {
        let person = Person { name: String::from("Alice") };
        let dog = Dog { name: String::from("Buddy") };
    
        // Greet 트레이트를 구현한 타입에 대해 print_greeting 함수 호출
        print_greeting(person);
        print_greeting(dog);
    }
    ```
    
    - trait를 사용하면 공통 속성에 대한 기능 정의가 가능함.
    - trait를 정의하고 사용할 구조체를 지정하면 상속과 비슷한 효과를 낼 수 있다.

## Enum

enum은 합타입으로 사용하기 좋다. 베리언트라고도 함.

- 정의
    
    ```rust
    // enum 키워드로 사용하고
    enum Message {
        Quit,
        Echo,
        Move,
        ChangeColor,
    }
    
    // 사용
    fn main() {
        println!("{:?}", Message::Quit);
        println!("{:?}", Message::Echo);
        println!("{:?}", Message::Move);
        println!("{:?}", Message::ChangeColor);
    }
    ```
    
- 다른 양의 값의 저장도 가능
    
    ```rust
    enum Message {
        Move { x: i32, y: i32 },
        Echo(String),
        ChangeColor(i32, i32, i32),
        Quit,
    }
    ```
    
- 선언적 상태 관리
    
    ```rust
    enum Status {
    	Init,
    	Loading,
    	Loaded(some data),
    	Error
    }
    
    fn main() {
    	...
    	const status = ...
    	
    	match status {
    		Status::Init => ...
    		Status::Loading => ... 
    		Status::Loaded(some data) => ...
    		Status::Error => ...
    	}
    }
    ```
    - 베리언트와 패턴매칭을 같이 사용하면 선언적인 상태 관리에 용이함.