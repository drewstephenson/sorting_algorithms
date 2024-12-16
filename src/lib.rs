use rand::Rng;

pub fn create_random_vector(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();  // Create random number generator in the current thread
    let mut random_vector = Vec::new();

    for _ in 0..size {   // for(i = 0; i < size; i++)
        random_vector.push(rng.gen_range(0..100)); // From 0 to n, generate numbers in range (0, 100)
    }

    random_vector
}

// Divide array into sorted and unsorted portion, set the first element as the minimum, compare it to unsorted elements.
// If a smaller element is found, swap it with the current minimum and continue.
// We want the vector passed to be a mutable reference to sort the elements in place.
// Warning: Your computer will not like it if you input an array with a big size.
pub fn selection_sort(vector: &mut Vec<i32>) {
    let mut min_index;
    let mut temp;
    let size = vector.len();

    for i in 0..size {  // 0 to i-1 is sorted, i to size-1 is unsorted
        min_index = i;
        for j in i+1..size {  // Check unsorted portion for an element smaller than current minimum
            if vector[j] < vector[min_index] { 
                min_index = j;
            }
        }
        temp = vector[i];
        vector[i] = vector[min_index];
        vector[min_index] = temp;
    }

}

// Divide into a sorted and unsorted portion, choose element, compare to each in sorted.
// If element in sorted is greater than chosen element, shift rest of sorted vector to right and insert chosen element.
pub fn insertion_sort(vector: &mut Vec<i32>) {
    let size = vector.len();

    for current in 1..size {
        let temp = vector[current];
        
        for i in (0..current).rev() {  // Loop from current-1 to 0
            if temp < vector[i] {
                vector[i + 1] = vector[i]; // shift i element
            }

            else { // current element is larger than vector[i], insert temp here.
                vector[i + 1] = temp;
                break;
            }
        }

        if vector[0] > temp {  // current element is the smallest in the sorted portion
            vector[0] = temp;
        }
    }
}

// Select a pivot, partition the array so that the elements less than the pivot are on the left and greater on the right.
// Recursively sort the left and right partitions.
pub fn quick_sort(vector: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end { // If the start index is set to be greater or equal to the end index, it should be done sorting.
        return;
    }

    let pivot = vector[(start + end) / 2];  // Choose random element in the middle (random) as pivot
    let mut left = start;
    let mut right = end;

    while left <= right { // Partitioning will go until the left and right pointers meet.
        if vector[left] < pivot { // Ignore elements smaller than pivot on left
            left += 1;
        }
        else if vector[right] > pivot && right > 0 { // Ignore elements larger than pivot on right
            right -= 1;
        }
        else {
            let temp = vector[left];
            vector[left] = vector[right];
            vector[right] = temp;
            left += 1;

            if right > 0 {
                right -= 1;
            }
        }
    }

    if start < right {
        quick_sort(vector, start, right);
    }
    if left < end {
        quick_sort(vector, left, end);
    }

}

// Recursively divides the arrays and puts them back together in sorted order
pub fn merge_sort(vector: &mut [i32]) {
    let size = vector.len();
    
    // Base case: if there's one or zero elements, it is done sorting
    if size <= 1 {
        return;
    }

    let middle = size / 2;
    let mut temp = vector.to_vec(); // Create the temporary vector before splitting

    let (first_half, second_half) = vector.split_at_mut(middle);

    // Recursively sort both halves
    merge_sort(first_half);
    merge_sort(second_half);
    
    // Call merge helper function on both halves
    merge(first_half, second_half, &mut temp);
    vector.copy_from_slice(&temp);
}

// Helper function for merge sort: Compares each element on left and right array and places them in new array in order
fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut result_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            result[result_index] = left[left_index];
            left_index += 1;
        } else {
            result[result_index] = right[right_index];
            right_index += 1;
        }
        result_index += 1;
    }

    // If any remaining in left array, add them in order
    while left_index < left.len() {
        result[result_index] = left[left_index];
        left_index += 1;
        result_index += 1;
    }

    // If any remaining in right, do the same
    while right_index < right.len() {
        result[result_index] = right[right_index];
        right_index += 1;
        result_index += 1;
    }
}

// Bubble sort: Compare each element with the next element and swap if the next element is smaller.
fn bubble_sort(vector: &mut Vec<i32>) {
    let size = vector.len();
    let mut swapped = true;

    while swapped {
        swapped = false;

        // For each element, compare with the next element and swap if the next element is smaller
        for i in 0..size - 1 {
            if vector[i] > vector[i + 1] {
                let temp = vector[i];
                vector[i] = vector[i + 1];
                vector[i + 1] = temp;
                swapped = true;
            }
        }
    }
}

#[derive(Debug)]
pub struct Options {
    pub size: usize,
    pub algorithm: String,
}

impl Options {
    pub fn build(size: usize, algorithm: String) -> Options {
        Options { size, algorithm }
    }
}

// Gets called from main, matches cmdl arguments to apply the correct sorting algorithm.
pub fn sort(options: Options) {
    let mut vector = create_random_vector(options.size);
    
    match options.algorithm.as_str() {
        // Selection Sort
        "s" => {
            println!("Performing selection sort...");
            selection_sort(&mut vector);
        }

        // Quick Sort
        "q" => {
            println!("Performing quick sort...");
            let size = vector.len() - 1;
            quick_sort(&mut vector, 0, size);
        }

        "i" => {
            println!("Performing insertion sort...");
            insertion_sort(&mut vector);
        }

        "m" => {
            println!("Performing merge sort...");
            merge_sort(&mut vector);
        }

        "b" => {
            println!("Performing bubble sort...");
            bubble_sort(&mut vector);
        }

        // Catch all, will return error message if other options are not matched.
        _ => {
            eprintln!("Please provide a valid algorithm type.");
            return;
        }
    }

    println!("Sorted vector: {:?}", vector);
}


