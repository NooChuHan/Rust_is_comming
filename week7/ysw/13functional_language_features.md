### 클로저와 반복자 정리

### 클로저(Closures)

클로저는 함수와 비슷한 익명 함수로, 환경에서 변수를 캡처할 수 있습니다. 이는 변수에 저장하거나 다른 함수에 인수로 전달할 수 있습니다.

### 환경 캡처

클로저는 정의된 스코프에서 값을 캡처할 수 있습니다. 예를 들어, 티셔츠 회사에서 프로모션을 위해 고객의 좋아하는 색상을 선택하거나 재고에서 가장 많은 색상을 선택하는 기능을 클로저로 구현할 수 있습니다.

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

```

### 클로저의 타입 추론과 명시

클로저는 함수와 달리 매개변수와 반환 값의 타입을 명시할 필요가 없습니다. 그러나 명시적으로 타입을 지정할 수도 있습니다.

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

```

### 클로저의 환경 캡처 방식

클로저는 환경을 불변 참조로 캡처, 가변 참조로 캡처, 소유권 이동으로 캡처할 수 있습니다.

- 불변 참조 캡처: 클로저 내부에서 환경의 값을 변경하지 않을 때 사용.
- 가변 참조 캡처: 클로저 내부에서 환경의 값을 변경할 때 사용.
- 소유권 이동: 클로저가 환경의 값을 소유해야 할 때 사용.

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("{:?}", list); // [1, 2, 3, 7]
}

```

### Fn 트레이트

클로저는 `Fn`, `FnMut`, `FnOnce` 트레이트를 구현할 수 있습니다.

- `Fn`: 클로저가 환경을 불변 참조로 캡처.
- `FnMut`: 클로저가 환경을 가변 참조로 캡처.
- `FnOnce`: 클로저가 환경의 값을 소유권 이동으로 캡처.

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}

```

### 반복자(Iterators)

반복자는 일련의 아이템들에 대해 순서대로 작업을 수행하는 패턴입니다. 러스트의 반복자는 게으릅니다. 즉, 소비되지 않으면 아무 작업도 하지 않습니다.

### 반복자의 기본 사용

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}

```

### 반복자 메서드

`Iterator` 트레이트는 `next` 메서드를 정의합니다.

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

```

### 반복자를 소비하는 메서드

- `sum`: 반복자의 모든 아이템을 합산.

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

```

### 반복자 어댑터

반복자 어댑터는 원본 반복자를 바꾸지만, 원본을 소비하지 않습니다. 예를 들어 `map` 메서드는 각 아이템에 대해 클로저를 호출하여 새로운 반복자를 생성합니다.

```rust
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);

```

### 환경을 캡처하는 클로저 사용하기

반복자 어댑터 메서드에 클로저를 전달하여 환경을 캡처할 수 있습니다.

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

```

### 성능 비교

반복자와 루프의 성능은 거의 동일합니다. 반복자는 고수준의 추상화를 제공하면서도 컴파일 시 최적화되어 런타임 성능에 영향을 주지 않습니다.

```rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}

```

### 결론

클로저와 반복자는 러스트에서 코드 재사용과 동작 커스터마이징을 가능하게 하는 강력한 도구입니다. 클로저는 환경을 캡처할 수 있어 함수보다 유연하며, 반복자는 일련의 아이템을 효율적으로 처리할 수 있습니다. 두 개념 모두 고수준의 추상화를 제공하면서도 성능을 희생하지 않습니다.

---

### [iterators1.rs](http://iterators1.rs/)

이 파일에서는 벡터에서 이터레이터를 사용하는 방법을 배우는 것이 목표입니다. 벡터를 이터레이터로 변환하고, `next` 메서드를 사용하여 요소를 하나씩 가져오는 것을 연습합니다.

```rust
#[test]
fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // 벡터를 이터레이터로 변환
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();

    // 이터레이터를 사용하여 요소를 하나씩 가져옴
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);
}

```

### [iterators2.rs](http://iterators2.rs/)

이 파일에서는 문자열을 변환하는 함수를 작성합니다.

```rust
// 첫 글자를 대문자로 변환하는 함수
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// 문자열 슬라이스를 받아서 첫 글자를 대문자로 변환한 벡터를 반환하는 함수
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

// 문자열 슬라이스를 받아서 첫 글자를 대문자로 변환한 문자열을 반환하는 함수
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|word| capitalize_first(word)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

```

### [iterators3.rs](http://iterators3.rs/)

이 파일에서는 나누기 연산을 수행하고, 나눌 수 없는 경우와 0으로 나누는 경우를 처리하는 함수를 작성합니다.

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// 주어진 두 수를 나눌 수 있는지 확인하고 결과를 반환하는 함수
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a % b != 0 {
        Err(DivisionError::NotDivisible(NotDivisibleError { dividend: a, divisor: b }))
    } else {
        Ok(a / b)
    }
}

// 나누기 결과를 벡터로 반환하는 함수
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

// 각 나누기 결과를 Result로 반환하는 함수
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list(), Ok(vec![1, 11, 1426, 3]));
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            list_of_results(),
            vec![Ok(1), Ok(11), Ok(1426), Ok(3)]
        );
    }
}

```

### [iterators4.rs](http://iterators4.rs/)

이 파일에서는 팩토리얼을 계산하는 함수를 작성합니다.

```rust
pub fn factorial(num: u64) -> u64 {
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

```

### [iterators5.rs](http://iterators5.rs/)

이 파일에서는 해시 맵을 사용하여 진행 상황을 추적하는 함수를 작성합니다.

```rust
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.values().filter(|&&v| v == value).count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection.iter().flat_map(|map| map.values()).filter(|&&v| v == value).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(1, count_iterator(&map, Progress::Some));
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(2, count_iterator(&map, Progress::None));
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state)
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(1, count_collection_iterator(&collection, Progress::Some));
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(4, count_collection_iterator(&collection, Progress::None));
    }

    #[test]
    fn count_collection_equals_for() {
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        let collection = get_vec_map();

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state)
            );
        }
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("

hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}

```
