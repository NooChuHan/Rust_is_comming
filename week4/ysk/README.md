- 크레이트
    - 러스트 코드를 묶을 수 있는 가장 작은 단위
- 바이너리 크레이트
    - 컴파일되어 바이너리 파일을 생성하는 크레이트
        - 생성: `cargo new <프로젝트명>`
        - 엔트리프인트: `main.rs`
- 라이브러리 크레이트
    - 다른 크레이트나 패키지에서 코드를 참조할 수 있도록 제공
    - 컴파일되지 않기 때문에 바이너리를 생성하지 않음 ( 다른 크리에이트 에서 참조해서 사용하게 하기 위함 )
        - 생성: `cargo new --lib`
        - 엔트리포인트: `lib.rs`
- 크레이트 루트
    - “컴파일 엔트리포인트”
    - 바이너리 크레이트 → `src/main.rs`
    - 라이브러리 크레이트 → `src/lib.rs`
- 모듈
    - 러스트는 파일 단위로, 또는 파일 하나에서도 여러 개의 모듈을 정의할 수 있다.
    - 공개 vs 비공개
        - 러스트의 모든 모듈 객체는 기본적으로 비공개
        - 외부에서 모듈에 접근하거나 모듈 내부의 객체에 접근을 허용하려면 `pub` 키워드를 사용
        
        ```rust
        pub fn make_sausage() {
            get_secret_recipe();
            println!("sausage!");
        }
        ```
        
    - 사용하기
        - `use`
            - 특정 경로를 현재 스코프로 가져오는 역할
        - `mod`
            - 해당 모듈을 사용하겠다고 선언하는 역할
    - mod를 정의 했을 때 찾아가는 과정
        1. `mod new_module` 를 같은 파일에 사용할 수 있는 모듈이 있는지 확인
        2. 같은 파일에 없다면, `src/new_module.rs` 파일을 찾아본다.
        3. `src/new_module` 폴더에서 [`mod.rs`](http://mod.rs) 파일을 찾아본다.
    - self
        
        ```rust
        mod mod2 {
            fn func() { // 1 
                println!("func");
            }
        
            mod mod1 {
                pub fn func() { // 2
                    println!("func");
                }
            }
        
            pub fn dummy() {
                func(); // 1
                self::func(); // 1
                mod1::func(); // 2
                self::mod1::func(); // 2
            }
        }
        ```
        
    - super
        
        ```rust
        mod mod1 {
            pub fn func() {
                println!("func");
            }
        }
        
        mod mod2 {
            use super::mod1;
        
            pub fn dummy() {
                mod1::func();
            }
        }
        
        ```
        
    - 패키지
        - 여러 크레이트를 모아 놓은 것
        - `Cargo.toml` 파일로 해당 패키지를 빌드하는 방법을 정의
        - 패키지는 아래와 같이 구성됨.
            - 1개의 라이브러리 크레이트
            - 여러 개의 바이너리 크레이트

