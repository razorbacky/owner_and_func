// Ownership and Function . 25.01.24
// 소유권과 함수

fn main() {
    let s = String::from("no copy String"); // s가 스코프 안으로 들어온다.

    takes_ownership(s); // s의 값이 함수로 이동한다.
                                    // ... 따라서 여기서부터는 s가 유효하지 않음

    let x = 5; // x가 스코프 안으로 들어온다.

    makes_copy(x); // x가 함수로 이동한다.
                                // i32는 Copy 이므로 계속 x를 사용할 수 있다.

    println!("origin x is : {x}"); // 확인을 위해 내가 직접 추가함, 기존의 x 값이 함수로 전달되었어도 그대로 출력됨.
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    let result = some_integer + 5; // 확인을 위해 내가 직접 추가함
    println!("{result}");
}