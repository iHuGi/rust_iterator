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

/// Demonstrates ownership transfer and consuming an iterator.
/// Takes ownership of `vec_a` (consuming it entirely) and mutably borrows `vec_b` 
/// to push the extracted elements into the destination.
fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

/// Demonstrates nested iteration and multiple heap allocations.
/// Takes an immutable slice and transforms it into a two-dimensional Vector,
/// where each inner Vector contains the individual characters (as Strings) of the original elements.
fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

/// Demonstrates searching and safely unwrapping optional values.
/// Iterates over an immutable slice to find a substring match, handling the resulting `Option`
/// by applying a transformation if found, or allocating a fallback String if not.
fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements.iter().find(|el| el.contains(search)).map_or(String::from(fallback), |el| el.to_string())
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

    // 4. Move elements from one vector to another, transferring ownership
    let mut destination_vec = vec![];
    move_elements(colors, &mut destination_vec);
    println!("Destination Vector after move:\n{:#?}", destination_vec);

    // Initialize a new, immutable heap-allocated vector of Strings
    let colors_v2 = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // 5. Explode the strings into a nested vector of characters
    let exploded = explode(&colors_v2);
    println!("Exploded Vector:\n{:#?}", exploded);

    // 6. Find a color containing a specific substring or return a fallback if not found
    let found_color = find_color_or(&colors_v2, "", "orange");
    println!("Found Color or Fallback:\n{}", found_color);
}