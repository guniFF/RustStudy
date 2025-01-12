use std::io; // io입출력 라이브러리 스코프 가져오기 -> std라고 불리는 표준 라이브러리에 있음
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("please input your guess.");

    // 변수의 값을 변경 가능하도록 하기위해 mut 추가
    // ::는 연관함수일때 사용
    let mut guess = String::new();

    // 여기부터 시작
    // 함수에서 파라미터를 넣을 때 해당 파라미터를 수정해야 한다면 &mut로 넣고 읽기만 한다면 &를 넣는다
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");

}

