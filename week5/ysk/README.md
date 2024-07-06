**Options**

- 정의 `Option<T>`는 값이 있거나 없을 수 있는 상황을 표현하는 열거형.
- 변형:
    - `Some(T)`: 값이 존재함
    - `None`: 값이 없음
- 활용
    
    ```rust
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    
    // pattern matching
    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        None => panic!("no match!"),
    }
    ```
    
    ```rust
    let optional_target = Some(target);
    
    // if let
    if let Some(word) = optional_target {
        assert_eq!(word, target);
    }
    ```
    

### Error

**에러의 종류**

복구가 가능한 에러 & 불가능한 에러

- 복구가 가능한 에러 (Recoverable Error)
    
    ```rust
    let f: Result<File, std::io::Error> = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("파일을 열 수 없습니다: {:?}", error);
        }
    };
    ```
    
- 불가능한 에러 (Unrecoverable Error)
    
    ```rust
    x.unwrap()
    ```
    

**Result Type**

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- Result 타입은 두 개의 베리언트를 가진 열거형(enum)
    - `T`는 성공 시 반환될 값의 타입.
    - `E`는 오류 시 반환될 값의 타입.
- `?`
    - 기본 기능:
        - `?` 연산자는 `Result<T, E>` 또는 `Option<T>` 타입에 사용
        - `Result`의 경우 `Err`를 만나면 즉시 그 오류를 반환
        - `Option`의 경우 `None`을 만나면 즉시 `None`을 반환
    - 동작 방식:
        - `Result`에서:
            - `Ok(value)` -> `value`를 추출
            - `Err(e)` -> 현재 함수에서 `Err(e)`를 즉시 반환
        - `Option`에서:
            - `Some(value)` -> `value`를 추출
            - `None` -> 현재 함수에서 `None`을 즉시 반환
    
    ```rust
    fn main() {
        let mut tokens = 100;
        let pretend_user_input = "8";
    
        let cost = total_cost(pretend_user_input)?;
    
        if cost > tokens {
            println!("You can't afford that many!");
        } else {
            tokens -= cost;
            println!("You now have {} tokens.", tokens);
        }
    }
    ```
    

**Unrecoverable Error**

- `panic!`
    - 직접 패닉을 발생
- `unwrap()`
    - Result나 Option이 Err나 None일 때 패닉을 발생
- `expect()`
    - unwrap과 유사하지만 사용자 지정 오류 메시지를 제공

```rust
fn main() {
    // 1. panic! 매크로 사용
    if true {
        panic!("This is a deliberate panic!");
    }

    // 2. unwrap() 사용
    let v = vec![1, 2, 3];
    v[99].unwrap(); // 인덱스가 범위를 벗어나 패닉 발생

    // 3. expect() 사용
    let user_input = "not a number";
    let number: i32 = user_input.parse().expect("Invalid number!");

    // 4. 배열 범위 초과 접근 (자동으로 panic 발생)
    let a = [1, 2, 3];
    let element = a[10]; // 이 라인에서 패닉 발생

    // 5. assert! 매크로 사용
    let x = 5;
    assert!(x == 10, "x is not 10!");
}
```

### traits

공유할 수 있는 메서드 집합을 정의하는 방법이다. 다른 프로그래밍 언어의 인터페이스와 유사한 개념.

- 트레이트를 extend 한 impl은 그 구현을 포함해야 한다.
    
    ```rust
    trait AppendBar {
        fn append_bar(self) -> Self;
    }
    
    impl AppendBar for String {
        fn append_bar(self) -> Self {
            let mut s = self;
            
            s.push_str("Bar");
    
            s
        }
    }
    ```
    
- 만약 구현이 트레이트 안에 있다면 포함 되지 않아도 된다.
    
    ```rust
    pub trait Licensed {
        fn licensing_info(&self) -> String {
            "Some information".to_string()
        }
    }
    
    struct SomeSoftware {
        version_number: i32,
    }
    
    struct OtherSoftware {
        version_number: String,
    }
    
    impl Licensed for SomeSoftware {} // Don't edit this line
    impl Licensed for OtherSoftware {} // Don't edit this line
    ```
    
- 하지만, impl에 세부 구현을 오버라이딩 할 수 있다.
    
    ```rust
    pub trait Licensed {
        fn licensing_info(&self) -> String {
            "Some information".to_string()
        }
    }
    
    struct SomeSoftware {
        version_number: i32,
    }
    
    struct OtherSoftware {
        version_number: String,
    }
    
    impl Licensed for SomeSoftware {
        // SomeSoftware에 대해 licensing_info 메서드 오버라이드
        fn licensing_info(&self) -> String {
            format!("SomeSoftware v{} is licensed under MIT.", self.version_number)
        }
    }
    
    impl Licensed for OtherSoftware {} // 기본 구현 사용
    ```
    
- 파라미터로 특정 트레이트를 구현하는 것을 받도록 요구 가능.
    
    ```rust
    fn compare_license_types(software: &dyn Licensed, software_two: &dyn Licensed) -> bool {
        // 두 소프트웨어의 라이선스 정보를 비교
        software.licensing_info() == software_two.licensing_info()
    }
    ```
    
    ```rust
    fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
        item.some_function() && item.other_function()
    }
    
    ```
    

### lifetimes

- 수명
    - 참조값이 필요한 만큼 유효하게 선언.
- 사실 모든 참조값에는 유효한 수명이 존재.
- 타입추론으로 타입선언을 생략할 수 있듯, 컴파일러의 Borrow Checker가 알아서 잘 처리해줌.
- 하지만, 참조간의 관계를 표현할 때 lifetime에 대한 정보를 요구하기도 함.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}
```