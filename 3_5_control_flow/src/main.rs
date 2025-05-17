fn main() {
    let condition = true;
    let mut num = if condition { 5 } else { 8 };

    while num <= 15 {
        println!("looping!");
        num += 1;
    }

    let storage = [5, 10, 15, 20];
    for element in storage {
        println!("element = {element}");
    }

    for index in (0..4).rev() {
        println!("storage({index}) = {}", storage[index]);
    }
}
