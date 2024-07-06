![image](https://github.com/NooChuHan/Rust_is_comming/assets/79236624/01826c5f-969b-4270-9b24-6783206ac46c)

## 1. errors1

```rust
// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        None
    } else {
        Some(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}

```

`Option` 대신 `Result`를 사용하여 오류 조건을 표현하도록 함수를 수정할 수 있습니다. `Result`는 성공 시 `Ok` 값을, 실패 시 `Err` 값을 반환할 수 있습니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 빈 이름은 허용되지 않습니다.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // 이 줄을 변경하지 마세요.
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}

```

### 설명

1. 함수의 반환 타입을 `Result<String, String>`로 변경합니다. 이는 성공 시 `String`을 반환하고, 실패 시 오류 메시지를 담은 `String`을 반환합니다.
2. `if` 조건문에서 `name`이 비어 있는 경우, `Err`를 반환합니다. 이 오류 메시지는 테스트에서 요구하는 메시지와 일치합니다.
3. `name`이 비어 있지 않으면, `Ok`를 반환하여 적절한 이름 태그 문자열을 반환합니다.

이 수정사항으로 인해 테스트가 올바르게 동작할 것입니다.

## 2. errors2

```rust
// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the items. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `total_cost` function on a string that is not a number, that function
// will return a `ParseIntError`, and in that case, we want to immediately
// return that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}

```

`total_cost` 함수는 주어진 문자열을 숫자로 파싱하고, 파싱이 성공하면 계산을 수행하고, 실패하면 오류를 반환해야 합니다. 이를 위해 `?` 연산자를 사용하여 오류를 간단히 전파할 수 있습니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?; // ? 연산자를 사용하여 오류를 전파합니다.

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}

```

### 설명

1. `item_quantity.parse::<i32>()?`:
   - `parse::<i32>()`는 `Result<i32, ParseIntError>` 타입을 반환합니다.
   - `?` 연산자는 `Result`가 `Ok`일 경우 값을 추출하고, `Err`일 경우 함수에서 해당 오류를 즉시 반환합니다.
   - 따라서, `qty`는 `item_quantity`가 올바르게 파싱되었을 경우에만 `i32` 값을 가지게 됩니다.
2. `Ok(qty * cost_per_item + processing_fee)`:
   - 파싱이 성공적으로 완료된 경우, 총 비용을 계산하여 `Ok`로 반환합니다.

이렇게 하면 `item_quantity`가 올바른 숫자일 때는 총 비용을 반환하고, 그렇지 않은 경우 `ParseIntError`를 반환하여 오류를 처리할 수 있습니다.

## 3. modules

```rust
// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::num::ParseIntError;

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

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

```

현재 `main` 함수에서 `?` 연산자를 사용하고 있습니다. 하지만 `?` 연산자는 함수가 `Result`를 반환할 때만 사용할 수 있습니다. `main` 함수는 기본적으로 `Result`를 반환하지 않으므로, 컴파일 오류가 발생합니다.

이를 해결하기 위해 두 가지 방법이 있습니다:

1. `main` 함수를 `Result`를 반환하도록 변경하는 방법.
2. `?` 연산자 대신 `match` 표현식을 사용하여 오류를 처리하는 방법.

여기서는 `main` 함수를 `Result`를 반환하도록 변경하는 방법을 사용해보겠습니다:

```rust
rust코드 복사
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

```

### 설명

1. `main` 함수의 반환 타입을 `Result<(), ParseIntError>`로 변경합니다. 이는 `main` 함수가 `Result`를 반환할 수 있게 하여 `?` 연산자를 사용할 수 있도록 합니다.
2. `total_cost` 함수 호출에서 `?` 연산자를 사용하여 오류가 발생하면 즉시 반환하도록 합니다.
3. `main` 함수의 끝에서 `Ok(())`를 반환하여 성공적으로 종료되었음을 표시합니다.

이렇게 수정하면 `main` 함수에서 `total_cost` 함수 호출 시 발생할 수 있는 오류를 적절히 처리할 수 있습니다.

## 4. errors

```rust
// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm... Why is this always returning an Ok value?
        Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

```

현재 `PositiveNonzeroInteger::new` 함수는 항상 `Ok` 값을 반환하고 있습니다. 이 함수는 입력 값이 양수인 경우에만 `Ok`를 반환하고, 그렇지 않은 경우에는 적절한 `CreationError`를 반환하도록 수정해야 합니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value > 0 {
            Ok(PositiveNonzeroInteger(value as u64))
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Err(CreationError::Negative)
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

```

### 설명

1. `PositiveNonzeroInteger::new` 함수 내에서 입력 값 `value`가 0보다 큰지 확인합니다.
   - 값이 0보다 크면 `Ok(PositiveNonzeroInteger(value as u64))`를 반환합니다.
2. 값이 0인지 확인합니다.
   - 값이 0이면 `Err(CreationError::Zero)`를 반환합니다.
3. 값이 0보다 작으면 `Err(CreationError::Negative)`를 반환합니다.

이렇게 수정하면, 입력 값이 양수일 때는 `Ok`를 반환하고, 0일 때는 `CreationError::Zero` 오류를, 음수일 때는 `CreationError::Negative` 오류를 반환하여 모든 테스트를 통과할 수 있습니다.

## 5. errors

```rust
// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// This exercise uses some concepts that we won't get to until later in the
// course, like `Box` and the `From` trait. It's not important to understand
// them in detail right now, but you can read ahead if you like. For now, think
// of the `Box<dyn ???>` type as an "I want anything that does ???" type, which,
// given Rust's usual standards for runtime safety, should strike you as
// somewhat lenient!
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The Box is declared as of type Box<dyn Trait> where Trait is
// the trait the compiler looks for on any value used in that context. For this
// exercise, that context is the potential errors which can be returned in a
// Result.
//
// What can we use to describe both errors? In other words, is there a trait
// which both errors implement?
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn ???>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}

```

이 문제에서는 `main` 함수가 여러 종류의 오류를 반환할 수 있도록 `Result`의 오류 타입을 일반화해야 합니다. 이를 위해 `Box<dyn std::error::Error>`를 사용하여 다양한 오류 타입을 처리할 수 있습니다.

수정된 코드는 다음과 같습니다:

```rust
rust코드 복사
use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}

```

### 설명

1. `main` 함수의 반환 타입을 `Result<(), Box<dyn std::error::Error>>`로 변경합니다. 이는 `main` 함수가 다양한 오류 타입을 반환할 수 있도록 해줍니다.
2. `pretend_user_input.parse()?`와 `PositiveNonzeroInteger::new(x)?`에서 발생할 수 있는 오류는 자동으로 `Box<dyn std::error::Error>`로 변환되어 반환됩니다.

이렇게 수정하면 `main` 함수는 다양한 오류 타입을 처리할 수 있으며, 컴파일이 성공할 것입니다.

## 6. errors

```rust
// errors6.rs
//
// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here, we
// define a custom error type to make it possible for callers to decide what to
// do next when our function returns an error.
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: add another error conversion function here.
    // fn from_parseint...
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let x: i64 = s.parse().unwrap();
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}

```

이 문제는 `parse_pos_nonzero` 함수에서 발생할 수 있는 두 가지 오류 유형 (`ParseIntError`와 `CreationError`)을 처리하는 것입니다. 이를 위해 커스텀 오류 타입 `ParsePosNonzeroError`를 정의하고, 각 오류 타입을 이 커스텀 오류 타입으로 변환하는 함수를 작성해야 합니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
use std::num::ParseIntError;

// 이 함수는 `parse_pos_nonzero`에서 발생할 수 있는 오류 유형을 정의합니다.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }

    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

// 이 아래의 코드는 변경하지 마세요.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // ParseIntError를 직접 생성할 수 없으므로 패턴 매칭을 사용해야 합니다.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}

```

### 설명

1. `ParsePosNonzeroError` enum에 `ParseIntError`를 처리할 수 있는 `from_parseint` 함수를 추가합니다.
2. `parse_pos_nonzero` 함수에서 문자열을 `i64`로 파싱할 때 `map_err`를 사용하여 발생한 오류를 `ParsePosNonzeroError::from_parseint`로 변환합니다.
3. `PositiveNonzeroInteger::new` 호출 시 발생할 수 있는 `CreationError`도 `map_err`를 사용하여 `ParsePosNonzeroError::from_creation`으로 변환합니다.

이렇게 하면 `parse_pos_nonzero` 함수에서 발생할 수 있는 두 가지 오류를 적절히 처리하고, 테스트를 통과할 수 있습니다.

---

# Genericse

## 1. Gennerices

```rust
// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let mut shopping_list: Vec<?> = Vec::new();
    shopping_list.push("milk");
}

```

Rust의 벡터는 특정 타입의 요소들을 저장합니다. 이 예제에서는 `shopping_list` 벡터에 문자열을 저장하려고 합니다. Rust에서 문자열은 `&str` 타입이므로, 벡터의 타입 매개변수에 `&str`을 지정해야 합니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}

```

### 설명

- `Vec<?>` 부분을 `Vec<&str>`로 변경합니다. 이로써 `shopping_list`는 `&str` 타입의 요소를 담는 벡터가 됩니다.
- `"milk"`는 `&str` 타입이므로 `shopping_list.push("milk");`는 올바르게 컴파일됩니다.

이제 이 코드는 올바르게 컴파일되고 실행됩니다.

## 2. Gennerices

```rust
// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Wrapper {
    value: u32,
}

impl Wrapper {
    pub fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

```

`Wrapper` 구조체를 제네릭으로 작성하여 어떤 타입의 값도 저장할 수 있도록 수정해보겠습니다.

먼저, `Wrapper` 구조체를 제네릭 타입 `T`를 사용하도록 변경하고, `new` 메서드도 이에 맞게 수정합니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

```

### 설명

1. `struct Wrapper<T>`: `Wrapper` 구조체를 제네릭 타입 `T`를 사용하도록 정의합니다.
2. `impl<T> Wrapper<T>`: `Wrapper`의 구현 블록을 제네릭 타입 `T`를 사용하도록 정의합니다.
3. `pub fn new(value: T) -> Self`: `new` 함수는 제네릭 타입 `T`의 값을 받아들입니다.
4. `Wrapper { value }`: `Wrapper` 구조체의 `value` 필드에 전달된 값을 저장합니다.

이제 `Wrapper`는 `u32`, `&str` 등 어떤 타입의 값도 저장할 수 있으며, 모든 테스트가 통과될 것입니다.

---

# traits

## traits1

```rust
// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}

```

`AppendBar` 트레이트를 구현하여 `String` 타입에 "Bar" 문자열을 추가하는 기능을 추가해보겠습니다.

`AppendBar` 트레이트의 `append_bar` 메서드를 `String` 타입에 대해 구현하면 됩니다. 이 메서드는 `self`를 받아서 "Bar"를 추가한 새로운 `String`을 반환합니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        format!("{}Bar", self)
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}

```

### 설명

1. `trait AppendBar`: 트레이트 정의.
2. `impl AppendBar for String`: `String` 타입에 대해 `AppendBar` 트레이트를 구현.
3. `fn append_bar(self) -> Self`: `append_bar` 메서드는 `self`를 받아서 "Bar"를 추가한 새로운 `String`을 반환.
   - `format!("{}Bar", self)`: `self` 문자열에 "Bar"를 추가하여 새로운 `String`을 생성.

이렇게 하면 `String` 타입에 대해 `AppendBar` 트레이트를 구현할 수 있으며, 주어진 테스트를 통과할 수 있습니다

## 2. traits

`AppendBar` 트레이트를 `Vec<String>` 타입에 대해 구현하여, 벡터의 끝에 "Bar" 문자열을 추가하는 기능을 구현해보겠습니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}

```

### 설명

1. `trait AppendBar` 정의:
   - `append_bar` 메서드는 트레이트에 속하며, 트레이트를 구현하는 타입의 인스턴스를 받아 `Self` 타입을 반환합니다.
2. `impl AppendBar for Vec<String>`:
   - `Vec<String>` 타입에 대해 `AppendBar` 트레이트를 구현합니다.
   - `fn append_bar(mut self) -> Self` 메서드는 가변(self)을 받아 벡터의 끝에 "Bar" 문자열을 추가한 후, 벡터를 반환합니다.
   - `self.push(String::from("Bar"))`: 벡터의 끝에 "Bar" 문자열을 추가합니다.

이렇게 하면 `Vec<String>` 타입에 대해 `AppendBar` 트레이트를 구현할 수 있으며, 주어진 테스트를 통과할 수 있습니다.

## 3. traits

```rust
// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub trait Licensed {
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}

```

`Licensed` 트레이트를 두 구조체(`SomeSoftware`와 `OtherSoftware`)에 대해 동일한 정보를 반환하도록 구현하려면 트레이트 자체에 기본 메서드를 정의할 수 있습니다. 이렇게 하면 각 구조체에서 동일한 메서드를 반복해서 작성할 필요가 없습니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
pub trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Some information")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}

```

### 설명

1. `Licensed` 트레이트에 기본 메서드 구현을 추가:
   - `fn licensing_info(&self) -> String`: 기본 구현을 제공하여 `String::from("Some information")`을 반환합니다. 이렇게 하면 이 트레이트를 구현하는 모든 타입은 별도의 메서드 구현 없이도 이 기본 메서드를 사용할 수 있습니다.
2. `impl Licensed for SomeSoftware` 및 `impl Licensed for OtherSoftware`에서 추가적인 메서드 구현이 필요하지 않습니다. 기본 메서드 구현을 자동으로 상속받습니다.

이렇게 하면 `SomeSoftware`와 `OtherSoftware`는 `Licensed` 트레이트를 구현하고, `licensing_info` 메서드는 기본적으로 동일한 정보를 반환합니다. 모든 테스트가 통과할 것입니다.

## 4. traits

```rust
// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn compare_license_types(software: ??, software_two: ??) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}

```

`compare_license_types` 함수는 `Licensed` 트레이트를 구현하는 두 개의 인수를 받아서 비교해야 합니다. 따라서 함수의 매개변수 타입으로 `impl Licensed`를 사용하면 됩니다. 이는 해당 함수가 `Licensed` 트레이트를 구현하는 타입들을 매개변수로 받을 수 있음을 나타냅니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}

```

### 설명

1. `compare_license_types` 함수의 매개변수 타입을 `impl Licensed`로 변경합니다. 이는 해당 함수가 `Licensed` 트레이트를 구현하는 어떤 타입이든 매개변수로 받을 수 있음을 나타냅니다.
2. 함수 내부에서 `software`와 `software_two`의 `licensing_info` 메서드를 호출하여 두 값을 비교합니다.

이렇게 하면 `compare_license_types` 함수는 `Licensed` 트레이트를 구현하는 타입을 받아서 라이선스 정보를 비교할 수 있으며, 주어진 테스트를 통과할 수 있습니다.

## 5. traits

```rust
// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func(item: ??) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}

```

`some_func` 함수는 `SomeTrait`와 `OtherTrait` 두 트레이트를 모두 구현하는 타입을 매개변수로 받아야 합니다. 이를 위해 트레이트 바운드(trait bounds)를 사용하여 함수 매개변수의 타입을 지정할 수 있습니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}

```

### 설명

1. `some_func` 함수의 매개변수 `item`의 타입을 제네릭 타입 `T`로 지정하고, `T`가 `SomeTrait`와 `OtherTrait` 두 트레이트를 모두 구현해야 한다고 명시합니다.
   - `fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool`: 이는 `T` 타입이 `SomeTrait`와 `OtherTrait` 두 트레이트를 모두 구현해야 한다는 것을 의미합니다.
2. 함수 내부에서 `item.some_function()`과 `item.other_function()`을 호출하여 두 함수의 반환값을 논리 AND 연산합니다.

이렇게 하면 `SomeStruct`와 `OtherStruct` 모두 `SomeTrait`와 `OtherTrait`를 구현하므로 `some_func` 함수를 호출할 수 있으며, 주어진 코드는 컴파일되고 실행될 것입니다.

---

# liftTimes

## 1.lifeTimes

```rust
// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}

```

Rust의 수명 주기(lifetime)을 명시하여 컴파일러가 참조의 유효성을 확인할 수 있도록 해야 합니다. `longest` 함수는 두 개의 문자열 슬라이스를 받아서 더 긴 문자열 슬라이스를 반환합니다. 반환되는 슬라이스의 수명을 명시해야 합니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}

```

### 설명

1. `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`:
   - `'a`는 수명 주기 파라미터입니다.
   - 함수에 전달되는 두 참조 `x`와 `y`는 동일한 수명 주기 `'a`를 가집니다.
   - 반환되는 참조도 동일한 수명 주기 `'a`를 가집니다.

이렇게 하면 `longest` 함수가 두 입력 참조와 동일한 수명을 갖는 참조를 반환하도록 명시하여, 컴파일러가 참조의 유효성을 확인할 수 있습니다. 이제 이 코드는 올바르게 컴파일되고 실행될 것입니다

## 2. lifetimes

```rust
// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}

```

문제의 핵심은 `result` 참조가 유효하지 않은 참조를 가지게 되는 것입니다. `string2`는 내부 스코프에서 선언되었고, 그 스코프가 끝나면 `string2`의 데이터는 무효화됩니다. 따라서 `result`는 무효화된 데이터를 참조하게 됩니다.

이를 해결하기 위해서는 `result`가 유효한 참조를 가질 수 있도록 해야 합니다. 여기서 간단한 방법은 `string2`를 `result`의 유효 범위와 동일한 범위로 이동하는 것입니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}

```

### 설명

1. `string2`의 선언을 `result`와 동일한 스코프로 이동했습니다. 이제 `string2`는 `result`가 유효한 범위 내에 존재하므로, `result`는 무효화된 데이터를 참조하지 않습니다.
2. `longest` 함수는 여전히 두 입력 참조와 동일한 수명을 가진 참조를 반환합니다.

이제 이 코드는 올바르게 컴파일되고 실행될 것입니다.

## 3. lifetimes

```rust
// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}

```

Rust의 구조체에서 참조를 저장할 때, 해당 참조의 유효 기간을 명시해야 합니다. 이는 구조체가 참조하는 데이터가 유효한 기간 동안 구조체도 유효하도록 하기 위함입니다.

여기서는 `Book` 구조체에 수명 주기를 추가해야 합니다. `Book` 구조체가 참조하는 `author`와 `title` 필드는 특정 수명을 가지므로, 이 수명을 명시합니다.

다음은 수정된 코드입니다:

```rust
rust코드 복사
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}

```

### 설명

1. `struct Book<'a>`: `Book` 구조체에 수명 주기 `'a`를 추가합니다. 이는 `Book` 구조체가 `'a`라는 수명 주기를 가지는 데이터에 대한 참조를 포함하고 있음을 의미합니다.
2. `author: &'a str`와 `title: &'a str`: `author`와 `title` 필드가 `'a` 수명을 가지는 `&str` 참조임을 명시합니다.

이렇게 하면 `Book` 구조체가 참조하는 데이터의 수명이 명시되어, 컴파일러가 참조의 유효성을 올바르게 확인할 수 있습니다. 이 코드는 이제 올바르게 컴파일되고 실행될 것입니다.
