/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 0;

    println!("n{length} = {n}", length = length + 1);

    while n != 1 {
        length += 1;

        if n % 2 == 0 {
            print!("{n} is even, so n{length} = {n} / 2 = ", length = length + 1);
            n = n / 2;
            println!("{n}");
        } else {
            print!("{n} is odd, so n{length} = 3 * {n} + 1 = ", length = length + 1);
            n = 3 * n + 1;
            println!("{n}");
        }
    }

    println!("The sequence terminates");
    length
}

fn main() {
    let n = 3;
    let finished = collatz_length(n);
    println!("The sequence finished at {finished} for n = {n}");
}