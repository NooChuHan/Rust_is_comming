use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
enum List {
    Cons(i32, Arc<Mutex<List>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // 초기 리스트를 생성합니다.
    let list = Arc::new(Mutex::new(Cons(1, Arc::new(Mutex::new(Cons(2, Arc::new(Mutex::new(Nil))))))));

    // 공유 리스트를 여러 스레드에 전달합니다.
    let list_clone_1 = Arc::clone(&list);
    let list_clone_2 = Arc::clone(&list);

    // 첫 번째 스레드에서 리스트를 수정합니다.
    let handle1 = thread::spawn(move || {
        println!("Thread 1 is modifying the list...");
        let mut list = list_clone_1.lock().unwrap();
        if let Cons(value, next) = &*list {
            let new_node = Cons(value + 10, Arc::clone(next));
            *list = new_node;
        }
    });

    // 두 번째 스레드에서 리스트를 수정합니다.
    let handle2 = thread::spawn(move || {
        println!("Thread 2 is modifying the list...");
        let mut list = list_clone_2.lock().unwrap();
        if let Cons(value, next) = &*list {
            let new_node = Cons(value * 2, Arc::clone(next));
            *list = new_node;
        }
    });

    // 두 스레드가 완료될 때까지 기다립니다.
    handle1.join().unwrap();
    handle2.join().unwrap();

    // 최종 리스트 상태를 출력합니다.
    println!("Final list: {:?}", list);
}
