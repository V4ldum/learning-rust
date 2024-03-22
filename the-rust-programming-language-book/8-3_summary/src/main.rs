use std::collections::HashMap;

fn main() {
    get_median_and_mode();
    convert_to_pig_latin();
}

fn get_median_and_mode() {
    let mut numbers1 = vec![1, 200, 200, 3, 40]; // median should be 40, mode should be 200
    let mut numbers2 = vec![100, 2, 30, 100, 4, 5000]; // median should be 65, mode should be 100

    // Median
    numbers1.sort();
    numbers2.sort();

    let median_numbers1 = get_median(&numbers1);
    let median_numbers2 = get_median(&numbers2);

    // Mode
    let mut numbers1_hashmap = HashMap::new();
    let mut numbers2_hashmap = HashMap::new();

    for n in &numbers1 {
        let entry = numbers1_hashmap.entry(*n).or_insert(0);
        *entry += 1;
    }
    for n in &numbers2 {
        let entry = numbers2_hashmap.entry(*n).or_insert(0);
        *entry += 1;
    }

    let max_numbers1 = get_max(numbers1_hashmap);
    let max_numbers2 = get_max(numbers2_hashmap);

    println!("The median for numbers1 is {}", median_numbers1);
    println!("The mode for numbers1 is {}", max_numbers1);
    println!("The median for numbers2 is {}", median_numbers2);
    println!("The mode for numbers2 is {}", max_numbers2);
}

fn get_max(hash_map: HashMap<i32, i32>) -> i32 {
    *hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| k)
        .unwrap()
}

fn get_median(v: &Vec<i32>) -> i32 {
    let len = v.len();
    if len % 2 == 0 {
        let middle = len / 2;
        (v[middle - 1] + v[middle]) / 2
    } else {
        v[len / 2]
    }
}

fn convert_to_pig_latin() {
    let mut string1 = String::from("first"); // should be irst-fay
    let mut string2 = String::from("apple"); // should be apple-hay

    pig_latin(&mut string1);
    pig_latin(&mut string2);

    println!("first is : {}", string1);
    println!("apple is : {}", string2);
}

fn pig_latin(string: &mut String) {
    match string.chars().nth(0).unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' => {
            string.push_str("-h");
        }
        c => {
            string.push_str(&*format!("-{c}").to_string());
            string.remove(0);
        }
    };

    string.push_str("ay");
}
