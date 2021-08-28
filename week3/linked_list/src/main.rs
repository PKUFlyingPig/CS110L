use linked_list::LinkedList;
use crate::linked_list::ComputeNorm;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<String> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    list.push_front("a".to_string());
    list.push_front("b".to_string());
    list.push_front("c".to_string());
    list.push_front("d".to_string());

    // test generics
    println!("===== test generics =====");
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display
    println!("\n");

    // test Clone trait
    println!("===== test clone trait =====");
    let mut list_copy = list.clone();
    list_copy.push_front("z".to_string());
    println!("original list : {}", list);
    println!("copied list : {}", list_copy);
    println!("\n");

    // test PartialEq
    println!("===== test clone trait =====");
    list.push_front("z".to_string());
    println!("original list : {}", list);
    println!("copied list : {}", list_copy);
    println!("original list == copied list is {}", list == list_copy);
    println!("\n");

    // println!("===== test IntoIterator for LinkedList<T> =====");
    // for val in list {
    //     print!("{}", val);
    // }
    // println!("original list : {}", list); this line should cause compile err

    println!("===== test IntoIterator for &LinkedList<T> =====");
    for val in &list {
        print!("{}", val);
    }
    println!("");
    println!("original list : {}", list);
    println!("\n");


    println!("===== test ComputeNorm =====");
    let mut f64_list: LinkedList<f64> = LinkedList::new();
    f64_list.push_front(3.0);
    f64_list.push_front(4.0);
    println!("{}", f64_list.compute_norm());
    println!("\n");
    // test ComputeNorm
    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
