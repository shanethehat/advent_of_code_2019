
/*
Fuel required to launch a given module is based on its mass. Specifically, to find the fuel 
required for a module, take its mass, divide by three, round down, and subtract 2.

For example:

For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
For a mass of 1969, the fuel required is 654.
For a mass of 100756, the fuel required is 33583.

Fuel itself requires fuel just like a module - take its mass, divide by three, round down, and 
subtract 2. However, that fuel also requires fuel, and that fuel requires fuel, and so on. Any 
mass that would require negative fuel should instead be treated as if it requires zero fuel; the 
remaining mass, if any, is instead handled by wishing really hard, which has no mass and is 
outside the scope of this calculation.

So, for each module mass, calculate its fuel and add it to the total. Then, treat the fuel amount 
you just calculated as the input mass and repeat the process, continuing until a fuel requirement 
is zero or negative. For example:

A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded 
down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel 
(654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which 
requires no further fuel. So, the total fuel required for a module of mass 1969 is 
654 + 216 + 70 + 21 + 5 = 966.
The fuel required by a module of mass 100756 and its fuel is: 
33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.

What is the sum of the fuel requirements for all of the modules on your spacecraft when also 
taking into account the mass of the added fuel? (Calculate the fuel requirements for each module 
separately, then add them all up at the end.)
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
        total += calculate_total_fuel_requirement(line.parse::<i32>().unwrap());
    }   

    println!("The total fuel requirements are {}", total);
}

fn calculate_total_fuel_requirement(mut mass: i32) -> i32 {
    let mut total: i32 = 0;
    while mass >= 8 {
        mass = calculate_single_fuel_requirement(mass);
        total += mass;
    }
    return total;
}

fn calculate_single_fuel_requirement(mass: i32) -> i32 {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_a_mass_that_divides_by_3() {
        assert_eq!(calculate_single_fuel_requirement(12), 2);
    }

    #[test]
    fn it_handles_a_mass_that_does_not_divide_by_3() {
        assert_eq!(calculate_single_fuel_requirement(16), 3);
    }

    #[test]
    fn it_does_not_add_fuel_for_a_tiny_mass() {
        assert_eq!(calculate_total_fuel_requirement(2), 0);
    }

    #[test]
    fn it_adds_fuel_weight() {
        assert_eq!(calculate_total_fuel_requirement(1969), 966);
    }
}