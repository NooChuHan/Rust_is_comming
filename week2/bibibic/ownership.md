>소유권: 힙에서 불필요한 데이터를 정리하고 중복되는 데이터를 최소화하는 등 힙 데이터를 관리하기 위한 개념 

# 소유권 규칙

- 각각의 값은 소유자(owner)가 정해져 있다.
- 한 값의 소유자는 하나 뿐이다.
- 소유자가 scope 밖으로 벗어나면 값은 버려진다. (dropped) 
  - 즉, 소유권은 scope 안에서만 유효하다.

##  String 타입

> String 타입은 힙에 할당된 데이터를 다루기 때문에 컴파일 타임에 크기를 알 수 없는 데이터를 저장할 수 있다.



**String 타입 생성하기**

`::` 은 타입에 있는 특정 함수를 지정하는 네임스페이스 연산자

``` rust
// String::from 함수는 필요한 만큼 메모리를 요청한다.
let s = String::from("test");

// push_str()로 문자열에 리터럴 추가
s.push_str(", hi!");

println!("{}", s);
```

메모리를 해제하려면 변수가 scope를 벗어나게 한다.

rust는 변수가 닫힌 중괄호 `}`가 나타나는 시점에 자동으로  `drop` 함수를 호출해 메모리를 해제한다.

``` rust
{
    let s = String::from("test");
    // ...
}

// scope 밖으로 벗어나면 s는 자동으로 메모리에서 해제된다.
```



# 이동

아래 예시처럼 s2에 s1을 얕은 복사하면 rust는 s1을 **유효하지 않은** 값으로 판단한다.

- s1과 s2는 같은 힙 데이터를 바라보고 있기 때문에, 각각 메모리를 해제하는 `중복 해제` 에러가 발생할 수 있다.
- 이를 방지하기 하고 메모리 안정성을 보장하기 위해 기존의 변수가 무효화된다.
-  = **이동**

```rust
let s1 = Strinig::from("hi");

// 이 뒤로 s1은 유효하지 않은 변수가 된다.
let s2 = s1;
```



# 클론

힙 데이터까지 복사하는 깊은 복사 방법

```rust
let s1 = String::from("test");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```



# 복사

힙이 아니라 스택에 저장되는 데이터(컴파일 타임에 크기가 고정되는 자료형. e.g. 정수형)는 `clone()`을 쓰지 않아도 데이터 채로 복사된다.

스택에 저장되는 타입은  `Copy` 트레이드가 구현되어 _이동_되지 않고 **복사**된다.

- int (e.g. u32)
- boolean
- float (e.g. f64)
- char
- Copy 가능한 타입만으로 구성된 tuple (e.g. `(i32, i32)`)



# 함수 

함수를 호출하는 과정에서 Copy가 아닌 값은 소유권이 이동된다.

```rust
fn main() {
	let s = String::from("test");

	// 변수 s이 함수로 이동되어
	takes_ownership(s)
	// s 값은 더 이상 유효하지 않음
    // (*)가 발생하면 메모리도 해제됨

	let num = 10;

	// num은 함수로 이동되지만
	makes_copy(num)
	// i32는 Copy이므로 여전히 num 사용 가능
}

fn takes_ownership(s: String) {}
// s가 스코프를 벗어나면서 drop()이 호출 -> 메모리 해제 (*)

fn makes_copy(num: i32) {}
```

함수를 반환하는 과정에서(대입) 소유권은 호출자 함수로 이동된다.

```rust
fn main() {
    // gives_ownership()은 반환값을 s1으로 이동
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    // takes_and_gives_back()은 반환값을 s3로 이동
    let s3 = takes_and_gives_back(s2); 
}
// s1: 스코프를 벗어나면서 dropped
// s2: 이동되어 변화 없음
// s3: 스코프를 벗어나면서 dropped

// 함수는 반환값을 호출자 함수로 이동시킨다.
fn gives_ownership() -> String {
    let s = String::from("yours");
    
	// s가 반환되어 호출자 함수 쪽으로 이동
    s
}

// 이 함수는 String을 취하고 같은 것을 반환합니다
fn takes_and_gives_back(ss: String) -> String {
    // ss가 반환되어 호출자 함수 쪽으로 이동
    ss
}
```



# 참조

함수에 값을 _이동_시키면서 기존 데이터를 사용할 수 없는 상황을 방지하기 위해 **참조**를 사용할 수 있다.

참조자는 동일 스코프 내에서 여럿 존재할 수 있다.

```rust
fn main() {
    let s1 = String::from("hello");

    // s1을 참조자로 넘겨줌
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// s를 참조자로 받아옴
fn calculate_length(s: &String) -> usize {
    s.len()
}
// s는 참조되기만 하고 소유되지는 않으므로 버려지지 않는다.
```



# 가변 참조자

참조된 값을 수정하기 위해서 사용한다. 단, **가변 참조자는 스코프 당 2개 이상 만들 수 없으며, 불변 참조자가 있을 때도 만들 수 없다.**

```rust
fn main() {
    let mut s = String::from("hello");

    // 가변 참조자는 &mut를 사용한다.
    change(&mut s);
    
    // 에러 발생
    let r = &mut s;
    
    let mut ss = String::from("world!");
    
    // 에러 발생
    let r1 = &mut ss;
    let r2 = &mut ss;
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

하지만 불변 참조자가  마지막으로 사용된 시점에서 더 이상 사용되지 않는다면 이를 컴파일러가 미리 알 수 있기 때문에 가변 참조자를 만들 수 있다.



# 댕글링 참조

어떤 메모리를 가리키는 포인터가 남아있는 상황에서 일부 메모리가 해제되면서 다른 개체가 할당 받았을지도 모르는 메모리를 참조하게 된  포인터를 말한다.

하지만 러스트는 참조자가 스코프를 벗어나기 전에 데이터가 먼저 스코프를 벗어나는지 컴파일러가 확인하여 댕글링 참조를 방지한다.

``` rust
fn main() {
    let reference_to_nothing = dangle();
}

// (틀린 케이스) Stirng의 참조자를 반환
fn dangle() -> &String {
    let s = String::from("hello");

    // String s의 참조자를 반환
    &s
}
// 스코프를 벗어났기 때문에 s의 메모리는 해제 -> error 발생

// (옳은 케이스)
fn no_dangle() -> String {
    let s = String::from("hello");
	// String을 직접 반환하여 값을 이동시켜야 한다.
    s
}
```



# 슬라이스

컬렉션의 연속된 일련 요소를 참조할 수 있게 해주며 소유권을 갖지 않는다.

```rust
let s = String::from("hello world");

// 0번째 인덱스에서 4번째 인덱스까지 슬라이스 생성
let hello = &s[0..5];
// 0부터 시작하는 경우에는 앞의 값 생략 가능
let hello2 = &s[..5];

// 6번째 인덱스에서 10번째 인덱스까지 슬라이스 생성
let world = &s[6..11];
// 마지막 인덱스에서 끝나는 경우는 뒤의 값 생략 가능
let world2 = &s[6..];

// 앞 뒤를 모두 생략하는 경우 전체 문자열이 슬라이스로 생성
let hello_world = &s[..];
```