# 05. Vecs

## 1. 벡터에 여러 값의 목록 저장하기

### 문제 코드

```rust
// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = vec![]; // a vector

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}

```

### 이유와 해결 방법

`vec![]` 매크로를 사용하여 벡터를 초기화하고, 배열과 동일한 요소를 포함하도록 수정해야 합니다.

### 해결 코드

```rust
fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = vec![10, 20, 30, 40]; // a vector

    (a, v)
}

```

`vec![]` 매크로를 사용하여 벡터를 배열과 동일한 요소로 초기화합니다.

## 2. 벡터 값에 대해 반복하기

### 문제 코드

```rust
// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        // 2, 4, 6, 8, 10
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        // 2, 4, 6, 8, 10
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}

```

### 이유와 해결 방법

첫 번째 함수에서는 벡터를 반복하고 각 요소를 2배로 만듭니다. 두 번째 함수에서는 `map` 함수를 사용하여 새로운 벡터를 생성합니다.

### 해결 코드

```rust
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        *element = *element * 2;
    }
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        element * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}

```

### 설명

- `mut v: Vec<i32>`: `mut` 키워드는 벡터 `v`를 수정할 수 있도록 합니다.
- `v.iter_mut()`: 벡터 `v`의 요소에 대한 가변 반복자를 반환합니다.
- `for element in v.iter_mut()`: 각 요소를 반복하면서 `element`가 각 항목에 대한 가변 참조가 됩니다.
- `element = *element * 2;`: `element`를 역참조하여 값을 수정합니다.
- `v.iter().map(|element| element * 2)`: 각 요소를 2배로 변환합니다.
- `.collect()`: 변환된 요소들을 새로운 벡터로 수집합니다.

Rust 개발자들 사이에서 `vec_map` 함수와 같은 패턴이 더 선호되는 경우가 많습니다. 이 패턴은 함수형 프로그래밍 원칙을 따르며, 가변 상태와 부작용을 피하여 코드를 더 예측 가능하고 이해하기 쉽게 만듭니다. 하지만 대규모 데이터 세트를 다루거나 성능이 중요한 경우에는 `vec_loop`와 같은 방식으로 원래 데이터를 직접 수정하는 것이 필요하거나 더 효율적일 수 있습니다. 두 패턴 모두 유용하며, 특정 상황과 요구 사항에 따라 선택하면 됩니다.

```rust
rust코드 복사
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        *element = *element * 2;
    }
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        element * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}

```

---

# 06.move_semantics

## 1. 벡터 업데이트하기

### 문제 코드

```rust
#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(88);
    vec
}

```

### 이유와 해결 방법

현재 `fill_vec` 함수는 벡터의 소유권을 가져옵니다. Rust에서 소유권을 함수에 넘기면, 원래 변수가 소유권을 잃어 더 이상 사용할 수 없습니다. 이 문제를 해결하기 위해서는 `fill_vec` 함수 내에서 벡터를 가변으로 선언해야 합니다.

```rust
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

```

`mut` 키워드를 사용하여 함수 내에서 벡터를 가변으로 선언함으로써 벡터를 수정할 수 있게 합니다.

## 2. 벡터의 깊은 복사

### 문제 코드

```rust
#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(88);
    vec
}

```

### 이유와 해결 방법

벡터를 함수에 전달할 때 원본 벡터를 복사하여 전달하면, 함수가 벡터의 소유권을 가져가더라도 원본 벡터는 영향을 받지 않습니다. `vec0`을 복사하여 `fill_vec`에 전달함으로써 원본 벡터를 유지할 수 있습니다.

```rust
fn main() {
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0.clone());

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

```

`vec0.clone()`을 사용하여 `vec0`의 깊은 복사본을 `fill_vec` 함수에 전달합니다. 이렇게 하면 `vec0`과 `vec1`은 서로 다른 벡터가 되어 `vec0`은 원래의 값을 유지합니다.

## 3. vec 매개변수의 가변화

### 문제 코드

```rust
#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

```

### 이유와 해결 방법

`fill_vec` 함수는 벡터를 받아서 수정해야 합니다. 이를 위해서는 `fill_vec` 함수의 매개변수를 가변 변수로 선언해야 합니다.

```rust
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

```

기존의 바인딩을 가변 바인딩으로 변경하여 `fill_vec` 함수에서 벡터를 수정할 수 있도록 합니다.

## 4. 함수 내에서 벡터 만들기

### 문제 코드

```rust
#[test]
fn main() {
    let vec1 = fill_vec();

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument - don't change this!
fn fill_vec() -> Vec<i32> {
    // Instead, let's create and fill the Vec in here - how do you do that?
    let mut vec = vec;

    vec.push(88);

    vec
}

```

### 이유와 해결 방법

벡터를 함수 내부에서 생성하여 반환함으로써 함수가 벡터의 소유권을 가지도록 합니다.

```rust
fn fill_vec() -> Vec<i32> {
    let mut vec = vec![22, 44, 66];
    vec.push(88);
    vec
}

```

`fill_vec` 함수는 이제 벡터를 직접 생성하고 요소를 추가한 후 반환합니다.

## 5. 가변 참조자

### 문제 코드

```rust
#[test]
fn main() {
    let x = 100;
    let y = &mut x;
    let z = &mut x;
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}

```

### 이유와 해결 방법

Rust에서는 동시에 여러 가변 참조자를 허용하지 않습니다. 이 문제를 해결하려면 가변 참조의 범위를 적절하게 설정해야 합니다.

```rust
#[test]
fn main() {
    let mut x = 100;
    {
        let y = &mut x;
        *y += 100;
    }
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}

```

각 가변 참조자가 속한 범위를 조정하여 컴파일 오류를 해결합니다. `y`의 가변 참조가 끝난 후 `z`를 생성합니다.

## 6. 참조와 대여

### 문제 코드

```rust
fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    data = &data.to_uppercase();

    println!("{}", data);
}

```

### 이유와 해결 방법

`get_char` 함수가 소유권을 가져가면 `data`가 이동되므로 `main` 함수에서 더 이상 `data`를 사용할 수 없습니다. 따라서 `get_char` 함수는 참조를 사용해야 합니다. 반면, `string_uppercase` 함수는 `data`의 소유권을 가져가야 합니다.

```rust
fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data.to_uppercase();

    println!("{}", data);
}

```

`get_char` 함수는 `data`의 참조를 받도록 수정하여 소유권을 가져가지 않게 합니다. 이렇게 하면 `main` 함수에서 `data`를 계속 사용할 수 있습니다. `string_uppercase` 함수는 여전히 `data`의 소유권을 가져가며, `data`를 대문자로 변환하고 출력합니다.

이렇게 각 문제를 해결하면 Rust의 소유권, 대여 및 가변 참조 개념을 더 깊이 이해할 수 있습니다.

---

# 07. Structs

### 1. 벡터에 여러 값의 목록 저장하기

```rust
// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct ColorClassicStruct {
    // TODO: Something goes here
}

struct ColorTupleStruct(/* TODO: Something goes here */);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        // let unit_like_struct =
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}

```

### 해결 방법

- `ColorClassicStruct`에 `red`, `green`, `blue` 필드를 추가합니다.
- `ColorTupleStruct`에 `(u8, u8, u8)` 필드를 추가합니다.
- `UnitLikeStruct`는 아무 필드도 필요 없습니다.

```rust
struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}

```

### 설명

- `ColorClassicStruct`는 전형적인 구조체 스타일로 필드 이름과 타입을 지정합니다.
- `ColorTupleStruct`는 튜플 스타일의 구조체로 필드에 이름 대신 타입만 지정합니다.
- `UnitLikeStruct`는 유닛 스타일의 구조체로 필드가 없습니다.

---

### 2. Order 구조체

```rust
// structs2.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        // TODO: Create your own order using the update syntax and template above!
        // let your_order =
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}

```

### 해결 방법

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        let your_order = Order {
            name: String::from("Hacker in Rust"),
            count: 1,
            ..order_template
        };

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}

```

### 설명

- `..order_template` 구문을 사용하여 템플릿 구조체의 나머지 필드를 복사합니다.
- 필요한 필드만 명시적으로 재정의하여 새로운 `Order` 인스턴스를 만듭니다.

---

### 3. Package 구조체

```rust
// structs3.rs
//
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
//
// Execute `rustlings hint structs3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
        if weight_in_grams < 10 {
            panic!("Can not ship a package with weight below 10 grams.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> ??? {
        // Something goes here...
    }

    fn get_fees(&self, cents_per_gram: u32) -> ??? {
        // Something goes here...
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}

```

### 해결 방법

```rust
impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
        if weight_in_grams < 10 {
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

```

### 설명

- `is_international` 메서드는 패키지가 국제적인지 여부를 판단합니다.
- `get_fees` 메서드는 그램당 요금을 받아 총 운송 요금을 계산합니다.

---

# 08. Enums

### 1. Message 열거형

```rust
// enums1.rs
//
// No hints this time! ;)

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

```

### 해결 방법

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

```

### 설명

- `Message` 열거형에 `Quit`, `Echo`, `Move`, `ChangeColor` 변형을 정의합니다.

---

### 2. Message 열거형과 메서드

```rust
// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

```

### 해결 방법

```rust
#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

```

### 설명

- `Message` 열거형에 `Move`, `Echo`, `ChangeColor`, `Quit` 변형을 정의합니다.
- `call` 메서드는 `Message`의 각 변형을 출력합니다.

---

### 3. State 구조체와 Message 처리

```rust
// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

enum Message {
    // TODO: implement the message variant types based on their usage below
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        // Remember: When passing a tuple as a function argument, you'll need extra parentheses:
        // fn function((t, u, p, l, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}

```

### 해결 방법

```rust
enum Message {
    ChangeColor(u8, u8, u8),
    Echo(String),
    Move(Point),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
            Message::Echo(s) => self.echo(s),
            Message::Move(p) => self.move_position(p),
            Message::Quit => self.quit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}

```

### 설명

- `Message` 열거형에 `ChangeColor`, `Echo`, `Move`, `Quit` 변형을 정의합니다.
- `State` 구조체의 `process` 메서드는 `match` 표현식을 사용하여 각 `Message` 변형을 처리합니다.

---

# 인증
![image](https://github.com/NooChuHan/Rust_is_comming/assets/79236624/bc962662-be0a-4a25-a32c-4f0a67c8dacd)
