use std::collections::HashMap;

pub fn median(numbers: &[i32]) -> f64 {
    let mut numbers = Vec::from(numbers);
    numbers.sort();
    if numbers.len() % 2 == 1 {
        numbers[numbers.len() / 2] as f64
    } else {
        (numbers[numbers.len() / 2] + numbers[numbers.len() / 2 - 1]) as f64 / 2f64
    }
}

pub fn mode(numbers: &[i32]) -> Vec<i32> {
    let mut counts = HashMap::new();

    for num in numbers {
        let count = counts.entry(*num).or_insert(0u32);
        *count += 1;
    }

    let mut result = Vec::new();
    let mut current = 0;
    for (n, c) in counts {
        if current < c {
            current = c;
            result.clear();
            result.push(n);
        } else if current == c {
            result.push(n);
        }
    }

    if result.len() == numbers.len() {
        return Vec::new();
    }

    result.shrink_to_fit();
    result
}
