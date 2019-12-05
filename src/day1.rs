
/*
Fuel required to launch a given module is based on its mass. Specifically, to find the fuel 
required for a module, take its mass, divide by three, round down, and subtract 2.

For example:

For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
For a mass of 1969, the fuel required is 654.
For a mass of 100756, the fuel required is 33583.

The Fuel Counter-Upper needs to know the total fuel requirement. To find it, individually 
calculate the fuel needed for the mass of each module (your puzzle input), then add together 
all the fuel values.

What is the sum of the fuel requirements for all of the modules on your spacecraft?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME: &'static str = "files/day1.txt";

pub fn run() {
    let fh = File::open(FILENAME).expect("Unable to open file");
    let reader = BufReader::new(fh);

    let mut total: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        total += calculate_fuel_requirement(line.parse::<i32>().unwrap());
    }   

    println!("The total fuel requirements are {}", total);
}

fn calculate_fuel_requirement(mass: i32) -> i32 {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_a_mass_that_divides_by_3() {
        assert_eq!(calculate_fuel_requirement(12), 2);
    }

    #[test]
    fn it_handles_a_mass_that_does_not_divide_by_3() {
        assert_eq!(calculate_fuel_requirement(16), 3);
    }
}