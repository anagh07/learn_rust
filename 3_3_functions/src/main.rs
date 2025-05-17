fn main() {
    let a = 5;
    let b = 4;
    let result = plus(a, b);
    println!("Calculated: {a} + {b} = {result}");
}

fn plus(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
