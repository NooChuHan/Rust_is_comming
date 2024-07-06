# 해시맵

## 라이브러리

``` rust
use std::collections::HashMap;
```

## 생성

```rust
// hashmap 생성
let mut scores = HashMap::new();

// 삽입
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 접근
let team_name = String::from("Blue");

// get()은 Option<&V>를 반환
let score = scores.get(&team_name).copied().unwrap_or(0);
```

> `get` 메서드는 `Option<&V>`를 반환
>
> - 값이 있으면 -> `Some(V)` 반환
> - 값이 없으면 `None` 반환

> `unwrap_or` 메서드는 해시맵에 키가 할당되어 있지 않은 경우 특정 값을 할당할 수 있다.

``` rust
// for문으로 접근  
for (key, value) in &scores {
    println!("{key}: {value}");
}
```

## 소유권

해시맵에 키와 값을 삽입하면 소유권이 이전된다. 

```rust
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// 여기서부터 field_name과 field_value는 유효하지 X
```

참조자를 사용할 경우 값이 해시맵으로 이동되지 않아 계속 유효하지만, 해시맵이 유효한 동안 해당 값도 계속 유효해야 한다.

## 업데이트

``` rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
// 같은 키에 값을 삽입할 경우 마지막에 삽입한 값으로 덮어씌워짐
scores.insert(String::from("Blue"), 25);

// Blue: 25
println!("{:?}", scores);
```

`or_insert`  메서드를 쓰면 해시맵에서 해당 키가 있는지 검사하고 키가 존재할 경우에는 값을, 없는 경우에는 새 값을 삽입한다.

``` rust
let mut scores2 = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
// Blue는 이미 값이 있기 때문에 덮어씌워지지 않음
scores.entry(String::from("Blue")).or_insert(50);

// Blue: 10, Yello: 50
println!("{:?}", scores)
```

