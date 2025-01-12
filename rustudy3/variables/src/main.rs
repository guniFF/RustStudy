fn main() {
    // 변수와 가변성
    // let x = 5; // x에 2번 값을 넣을 수 없다.
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    // 상수
    const THERE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("const value: {THERE_HOURS_IN_SECONDS}");

    // 섀도잉
    // mut에 경우 값을 재할당 할 때 값의 메모리주소가 변하지 않지만 - 타입 변경 불가
    // let를 사용 하여 재할당을 하면 값의 메모리주소가 바뀐다 - 타입 변경 가능
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x  in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
