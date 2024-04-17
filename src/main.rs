use sorting_library::sorting;

fn main() {
    let mut numbers = vec![4, 2, 1, 3];
    println!("Original Numbers: {:?}", numbers);

    sorting::quick_sort(&mut numbers);
    println!("After Quick Sort: {:?}", numbers);

    let mut numbers = vec![4, 2, 1, 3];
    sorting::selection_sort(&mut numbers);
    println!("After Selection Sort: {:?}", numbers);

    let mut numbers = vec![4, 2, 1, 3];
    sorting::insertion_sort(&mut numbers);
    println!("After Insertion Sort: {:?}", numbers);

    let mut numbers = vec![4, 2, 1, 3];
    sorting::merge_sort(&mut numbers);
    println!("After Merge Sort: {:?}", numbers);
}
