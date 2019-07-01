pub fn nth(n: u32) -> u32 {
    // use a loop to generate all the numbers
    // use an inner for loop to test for prime number
    // increase the index in the inner loop
    // test for the nth prime in outer loop
    let mut starting_number = 2;
    let mut index = 0;
    let mut is_prime: bool = true;

    loop {
        for number in (2..starting_number).rev() {
            if starting_number % number == 0 {
                // if the number before it divides it
                // then it is not a prime number
                is_prime = false;
                break;
            }
            // means the number wasn't divided by any number
            // below it yet
            is_prime = true;
        }

        // check if the number in last sequence was prime
        if is_prime {
            if n == index {
                println!("Found it {}", starting_number);
                return starting_number;
            }
            index += 1;
        }
        // proceed to the next number
        starting_number += 1;
    }
}

// I found this very elegant solution and learnt a lot from it

pub fn is_prime(n: u32) -> bool {
    // return a boolean, which means
    // create a range of numbers from 2 to n-1
    // use a method any to check if any of them matches
    // the condition from the closure function which will be called
    // for every element in the range until the condition is met
    ! (2..n - 1).any(|i| n % i == 0)
}

pub fn nth(n: u32) -> Option<u32> {
    match n {
        n if n <= 0 => None,
        n => (1..).filter(|c| is_prime(*c)).nth(n as usize),
    }
}