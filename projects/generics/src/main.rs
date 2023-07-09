fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![1, 20, 100, 100, 10000];

    let largest = largest(&number_list);

    println!("largest: {}", largest);
}
