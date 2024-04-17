# Sorting Library

## Usage

This library provides sorting functions for various types of data. You can use it by importing the `sorting` module and calling the desired sorting functions.

```rust
use sortable_library::sorting;

fn main() {
    // Example usage
    let mut numbers = vec![4, 2, 1, 3];
    sorting::quick_sort(&mut numbers);
    println!("Sorted Numbers: {:?}", numbers);
}
```

![Demo Screenshot](./Screenshot(56))

## Examples
Sorting Numbers
```rust
use sortable_library::sorting;

fn main() {
    let mut numbers = vec![4, 2, 1, 3];
    
    println!("Original Numbers: {:?}", numbers);

    sorting::quick_sort(&mut numbers);
    println!("After Quick Sort: {:?}", numbers);

    sorting::selection_sort(&mut numbers);
    println!("After Selection Sort: {:?}", numbers);

    sorting::insertion_sort(&mut numbers);
    println!("After Insertion Sort: {:?}", numbers);

    sorting::merge_sort(&mut numbers);
    println!("After Merge Sort: {:?}", numbers);
}
```

## Lisence

This project is not licensed.



