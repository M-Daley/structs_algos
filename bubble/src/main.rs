fn main() {
    let v = [5, 2, 100, 5, 2];
    let r = [0, 0, 11, 2, 3, 10];

    eprintln!("Unsorted v:\t{:?}", v);
    let v = bubble_sort(&v);
    eprintln!("Sorted v:\t{:?}\n", v);

    eprintln!("Unsorted r:\t{:?}", r);
    let r = bubble_sort(&r);
    eprintln!("Sorted r:\t{:?}", r);
}

fn bubble_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
    let mut result: Vec<T> = collection.into();
    for _ in 0..result.len() {
        let mut swaps = 0;
        for i in 1..result.len() {
            if result[i - 1] > result[i] {
                result.swap(i - 1, i);
                swaps += 1;
            }
        }
        if swaps == 0 {
            break;
        }
    }
    result
}
