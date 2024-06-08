## 특징
- 컴파일 언어
- 강타입 언어이면서 정적 타입 언어
- GC를 사용하지 않음 (Ownership 모델을 쓴다고 하는데, 알아봐야 할 듯)

## variables
```rust
// 타입 추론을 사용한 변수 선언
let number = 10;
let name = "Rust";

// 직접 타입을 명시한 변수 선언
let number: i32 = 10;
let name: &str = "Rust";

// 가변 변수 선언
let mut mutable_number = 20;
mutable_number = 30; // 가능

// 상수 선언
const MAX_POINTS: u32 = 100_000;

// 변수 섀도잉
let shadow = 5;
let shadow = shadow + 1; // shadow의 값은 6
```
- `let`과 `const`로 변수를 선언한다. (타입 추론 기능도 제공된다.)
- 직접적인 타입 명시의 경우 다른 언어와 같이 `:`으로 한다.
- 네이밍 컨벤션 
  - 변수: snake case
  - 함수: snake case
  - 클래스: upper camel case
  - 상수: scream snake case
- 기본적으로 모든 변수는 "불변"이다. 하지만, 가변으로 만드려면 `mut` 키워드를 사용해라.
- 동일한 이름의 변수를 선언할 수는 있지만, 섀도잉 된다.

## functions
```rust
// 기본 함수 선언
fn add(x: i32, y: i32) -> i32 {
    x + y // return 키워드 생략
}

fn main() {
    let sum = add(5, 3);
    println!("Sum: {}", sum);

    // 익명 함수(클로저) 선언
    let multiply = |x: i32, y: i32| -> i32 {
        x * y
    };

    let product = multiply(4, 2);
    println!("Product: {}", product);
}
```
- 선언시 `fn` 키워드를 사용해서 선언
- 매개변수와 리턴 타입을 명시해줘야 한다.
- 마지막에 `return ` 키워드를 생략할 수 있다.
- 익명함수(클로저)도 정의 할 수 있다. ( `| |` 사이에 파라미터를 선언하고 리턴타입을 정해주면 됨. )

## if
```rust
fn main() {
    // 기본 if, else if, else 조건문 사용
    let number = 7;

    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is equal to 5");
    } else {
        println!("Number is greater than 5");
    }

    // if, else if, else를 표현식으로 사용
    let condition = true;
    let number = if condition { 5 } else { 10 };

    println!("The value of number is: {}", number);
}

```
- `if`, `else if`, `else` 가 있음.
- `if`, `else if`, `else`를 표현식으로 사용할 수 있다. ( let if )

## type
### 기본 데이터 타입

1. **정수형 (Integer)**
    - **부호 있는 정수형 (Signed Integers)**
        - `i8`, `i16`, `i32`, `i64`, `i128`, `isize`: 부호 있는 정수형은 음수와 양수를 모두 표현할 수 있습니다.
        - 여기서 `i`는 integer(정수)를 의미하며, 숫자는 비트 수를 나타냅니다.
        - 예를 들어, `i8`은 8비트 정수로 -128에서 127까지 표현할 수 있습니다.
        - `isize`는 시스템 아키텍처에 따라 크기가 달라지는 정수형으로, 32비트 시스템에서는 `i32`, 64비트 시스템에서는 `i64`와 동일합니다.
        
        | 타입  | 비트 수 | 최소 값              | 최대 값              |
        |-------|---------|----------------------|----------------------|
        | `i8`  | 8       | -128                 | 127                  |
        | `i16` | 16      | -32,768              | 32,767               |
        | `i32` | 32      | -2,147,483,648       | 2,147,483,647        |
        | `i64` | 64      | -9,223,372,036,854,775,808 | 9,223,372,036,854,775,807 |
        | `i128`| 128     | -170,141,183,460,469,231,731,687,303,715,884,105,728 | 170,141,183,460,469,231,731,687,303,715,884,105,727 |
        | `isize`| 시스템에 따라 다름 | 시스템에 따라 다름 | 시스템에 따라 다름 |

    - **부호 없는 정수형 (Unsigned Integers)**
        - `u8`, `u16`, `u32`, `u64`, `u128`, `usize`: 부호 없는 정수형은 0과 양수만 표현할 수 있습니다.
        - 여기서 `u`는 unsigned(부호 없음)를 의미하며, 숫자는 비트 수를 나타냅니다.
        - 예를 들어, `u8`은 8비트 정수로 0에서 255까지 표현할 수 있습니다.
        - `usize`는 시스템 아키텍처에 따라 크기가 달라지는 정수형으로, 32비트 시스템에서는 `u32`, 64비트 시스템에서는 `u64`와 동일합니다.

        | 타입  | 비트 수 | 최소 값 | 최대 값              |
        |-------|---------|---------|----------------------|
        | `u8`  | 8       | 0       | 255                  |
        | `u16` | 16      | 0       | 65,535               |
        | `u32` | 32      | 0       | 4,294,967,295        |
        | `u64` | 64      | 0       | 18,446,744,073,709,551,615 |
        | `u128`| 128     | 0       | 340,282,366,920,938,463,463,374,607,431,768,211,455 |
        | `usize`| 시스템에 따라 다름 | 0 | 시스템에 따라 다름 |

2. **부동 소수점형 (Floating-Point)**
    - `f32`: 32비트 부동 소수점형
    - `f64`: 64비트 부동 소수점형 (기본형)

3. **불리언형 (Boolean)**
    - `bool`: 참(`true`) 또는 거짓(`false`)

4. **문자형 (Character)**
    - `char`: 유니코드 스칼라 값으로 표현되는 문자 (4바이트)

### 복합 데이터 타입

1. **튜플 (Tuple)**
    - 서로 다른 타입의 값을 함께 저장
    ```rust
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    ```

2. **배열 (Array)**
    - 같은 타입의 값을 고정된 길이로 저장
    ```rust
    let array: [i32; 3] = [1, 2, 3];
    ```

3. **슬라이스 (Slice)**
    - 배열이나 벡터의 부분을 참조
    ```rust
    let slice: &[i32] = &array[1..3];
    ```

### 집합체 타입 (Compound Types)

1. **구조체 (Struct)**
    - 관련된 데이터 필드를 그룹화
    ```rust
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user = User {
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };
    ```

2. **열거형 (Enum)**
    - 여러 변형(variants)을 가질 수 있는 타입
    ```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let message = Message::Move { x: 10, y: 20 };
    ```

### 참조와 포인터

1. **참조 (Reference)**
    - 값의 위치를 가리킴 (`&T`, `&mut T`)
    ```rust
    let reference: &i32 = &5;
    let mut mutable_reference: &mut i32 = &mut 10;
    ```

2. **스마트 포인터 (Smart Pointer)**
    - 추가적인 메타데이터와 기능을 제공
    - `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>`, `Mutex<T>` 등

### 함수 타입

1. **함수 포인터**
    - 함수를 가리킬 수 있는 타입
    ```rust
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    let f: fn(i32) -> i32 = add_one;
    ```

2. **클로저 (Closure)**
    - 주변 환경을 캡처할 수 있는 익명 함수
    ```rust
    let closure = |x: i32| -> i32 { x + 1 };
    ```

### 사용자 정의 타입

1. **타입 별칭 (Type Alias)**
    - 복잡한 타입에 별칭을 부여
    ```rust
    type Kilometers = i32;
    ```

### 기타

1. **네버 타입 (Never Type)**
    - `!`: 값이 결코 반환되지 않음을 나타냄
    ```rust
    fn foo() -> ! {
        panic!("This call never returns.");
    }
    ```

2. **유닛 타입 (Unit Type)**
    - `()`: 값이 없음을 나타냄
    ```rust
    let unit: () = ();
    ```
