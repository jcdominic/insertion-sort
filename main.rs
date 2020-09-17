fn main() {
    let list = vec![2, 5, 3, 4, 1];
    let sorted_list = insertion_sort(list);
    println!("Sorted List: {:?}", sorted_list);
}

fn insertion_sort(mut list: Vec<usize>) -> Vec<usize> {
    for i in 0..(list.len()) {
        let mut j = i;
        while j > 0 && list[j - 1] > list[j] {
            let t = list[j];
            list[j] = list[j - 1];
            list[j - 1] = t;
            j -= 1;
        }
    }
    list
}
