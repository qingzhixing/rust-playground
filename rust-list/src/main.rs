mod list;

fn main() {
    let mut list = list::List::<i32>::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("length: {}", list.len());
    println!("to string: {}", list.to_string());
}
