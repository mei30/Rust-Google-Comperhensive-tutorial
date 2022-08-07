extern crate merge_sort;

fn main()
{
    let mut items = vec![3, 2, 1];

    println!("Before Sort\n");

    for v in &items
    {
        println!("{}", v);
    }

    let length = items.len();

    merge_sort::merge_sort::sort(&mut items, 0, length - 1);

    println!("After Sort\n");

    for v in &items
    {
        println!("{}", v);
    }
}