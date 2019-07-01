pub fn raindrops(n: u32) -> String {
    // Hold the values in an array
    let values = ["Pling", "Plang", "Plong"];

    // Create an empty string to hold the values
    let mut result = String::new();
    let mut factor = 3;

    for element in values.iter() {
        if n % factor == 0 {
            result += element;
        }
        factor += 2;
    }

    if result.len() > 1 {
        result
    } else {
        n.to_string()
    }
}
