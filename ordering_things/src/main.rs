fn main() {
    let a = [1, 222, 903, 35, 5, 1, 288, 2, 0, 1, 7675, 3];
    let b = [9, 651, 1, 88, 32, 654, 8, 77, 9, 1, 100, 2];

    eprintln!("Unsorted a:\t\t{:?}", a);
    let a = shell_sort_ascend(&a);
    eprintln!("Ascending Sorted a:\t{:?}\n", a);

    eprintln!("Unsorted b:\t\t{:?}", b);
    let b = shell_sort_des(&b);
    eprintln!("Decending Sorted b:\t{:?}", b);
}

fn shell_sort_ascend<T: PartialOrd + Clone>(collections: &[T]) -> Vec<T> {
    let n = collections.len();
    let mut gap = n / 2;
    let mut result: Vec<T> = collections.into();

    while gap > 0 {
        for i in gap..n {
            let temp = result[i].clone();

            let mut j = i;
            while j >= gap && result[j - gap] > temp {
                result[j] = result[j - gap].clone();
                j -= gap;
            }
            result[j] = temp;
        }
        gap /= 2;
    }
    result
}

fn shell_sort_des<T>(collections: &[T]) -> Vec<T>
    where
        T: PartialOrd + Clone,    
{
    let n = collections.len();
    let mut gap = n / 2;
    let mut result: Vec<T> = collections.into();
    
    while gap > 0 {
        for i in gap..n {
            let tmp = result[i].clone();

            let mut j = i;
            while j >= gap && result[j - gap] < tmp {
                result[j] = result[j - gap].clone();
                j -= gap;
            }
            result[j] = tmp;
        }
        gap /= 2;
    }
    result
}