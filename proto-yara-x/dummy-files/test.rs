fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number text here is {}", result);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}
