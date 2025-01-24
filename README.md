[Rust] Ownership and Function (소유권과 함수)
===

Copy 가능한 타입이 아닌 경우에는 소유권이 이전된 이후에는 사용할 수 없다.
그러나, Copy 가능한 타입인 경우에는 소유권이 이전된 경우라도 계속 사용할 수 있다.

```rust
fn main() {
    let s = String::from("No Copy String");

    takes_ownership(s);
    /* 변수 s 값은 takes_ownership(s) 로 소유권이 이전되었고, Copy 가능한 타입이 아닌 String 으로, 소유권이 함수로 이전되어 재사용이 불가능하다. */

    let x = 5;

    makes_copy(x);
    /* 변수 x는 정수형 타입인 i32 이며, i32는 Copy 가능한 타입이기 때문에 함수로 소유권이 이전된 이후에도 계속 x 값을 사용할 수 있다. */

    println!("origin x is : {x}"); // "origin x is : 5" 가 출력됨
}

fn takes_ownership(some_string: String) {
    println!("{some_string}"); // "No Copy String" 이 출력됨
}

fn makes_copy(some_integer: i32) {
    let result = some_integer + 5;  // 기존 x와 함수에서 출력하는 some_integer의 비교를 위해 기존값에 5를 더함
    println!("{result}"); // some_integer의 인자인 x, 즉 5가 되고 some_integer + 5, 합이 "10" 이 출력됨
}
```

Rust 는 takes_ownership 함수를 호출한 이후부터 s 를 사용하려고 하는 경우, 컴파일 타임 에러가 발생한다.
이런 정적 검사들이 프로그래머의 실수를 방지하는 역할을 하며, 어느 지점에서 변수를 사용할 수 있고, 어느 지점에서 소유권 규칙이 제재하는지 확인하려면 main 함수에 s, x 변수를 사용하는 코드를 여기저기 추가하라.
