fn main() {
    let number = 3;

    /* rust는 boolean이 아닌 값을 자동으로 변환하지 않기 때문에 JS처럼 사용하면 에러 발생함 */
    // if number { ... }

    if number != 0 { 
        println!("{number} is not 0")
     }

    /* 
    if 표현식을 사용해서 변숫값을 할당할 수 있다. 
    단, if와 else에서 바인딩하려는 값은 같은 type이어야 한다.
    */
     let value = if number > 0 { number * 3 } else { 0 };
     println!("value: {value}")
}