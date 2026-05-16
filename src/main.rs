use std::vec;

/// Demonstrates immutable borrowing and functional iterator chains.
/// Takes a slice of Strings (`&[String]`) to borrow data without taking ownership.
fn print_elements(elements: &[String]) {
    // Traditional imperative approach (kept for reference):
    // for element in elements {
    //     println!("{}", element);
    // }

    // Functional approach: iterating, transforming, and consuming without mutation.
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

/// Demonstrates mutable borrowing and in-place data modification.
/// Takes a mutable slice (`&mut [String]`) to alter the original data directly in memory.
fn shorten_strings(elements: &mut [String]) {
    elements
        .iter_mut()
        .for_each(|el| el.truncate(1));
}

/// Demonstrates data transformation and allocation.
/// Takes an immutable slice, maps a transformation, and allocates a brand new Vector on the heap.
fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}

fn main() {
    // Initialize a mutable, heap-allocated vector of Strings
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // 1. Pass a partial, immutable slice (borrows elements at index 1 and 2: "green", "blue")
    print_elements(&colors[1..3]);

    // 2. Generate a new vector of uppercase strings, leaving the original `colors` untouched
    let uppercased = to_uppercase(&colors);
    println!("Uppercased Vector:\n{:#?}", uppercased);

    // 3. Mutate the original `colors` vector in place
    shorten_strings(&mut colors);
    println!("Mutated Original Vector:\n{:#?}", colors);
}