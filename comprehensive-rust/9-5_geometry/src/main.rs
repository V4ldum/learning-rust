type Point = [f64; 3];

// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.
fn magnitude(point: &Point) -> f64 {
    (point[0].powi(2) + point[1].powi(2) + point[2].powi(2)).sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
fn normalize(point: &mut Point) {
    let magnitude = magnitude(&point);
    point[0] /= magnitude;
    point[1] /= magnitude;
    point[2] /= magnitude;
}

// Use the following `main` to test your work.
fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}