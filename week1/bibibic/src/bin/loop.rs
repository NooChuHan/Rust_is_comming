fn loop_example() {
    /*
    - loop문은 while과 다르게 조건문 없이 사용되며, 내부에 break가 없을 경우 무한으로 루프한다.    
    loop {
        ...
        break;
    }
    */

    let mut count = 1;

    // loop문으로 변수에 값을 할당할 수 있다.
    let result = loop {
        count *= 2;
        
        if count > 10 {
            break count; // break할 때 원하는 값을 반환할 수 있다.
        }
    };

    println!("loop result: {result}");
}

fn loop_label_example() {
let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // 루프 라벨이 없는 break는 inner loop만 탈출한다.
            }
            if count == 2 {
                break 'counting_up; // 루프 라벨이 있는 outer loop를 탈출한다.
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_example() {
    /*
    while 조건문 {
        ...
    }
    */
}

fn for_example() {
    /*
    - 컬렉션에 대한 반복문
    for el in array {
        ...
    }

    - 반복 범위값 설정
    for i in (start..end) {
        ...
    }

    - 반복 범위값 설정 (역순)
    for i in (start..end).rev() {
        ...
    }
     */
}

fn main() {
    /*
    - loop
    - while
    -for
     */

    loop_example();
    loop_label_example();
}