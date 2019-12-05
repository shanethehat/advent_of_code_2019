/*
https://adventofcode.com/2019/day/2
*/

static ADDITION: usize = 1;
static MULTIPLICATION: usize = 2;
static END: usize = 99;

struct Operation {
    opcode: usize,
    source1: usize,
    source2: usize,
    target: usize,
}

pub fn run() {
    let input = String::from("1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,19,5,23,2,23,9,27,1,5,27,31,1,9,31,35,1,35,10,39,2,13,39,43,1,43,9,47,1,47,9,51,1,6,51,55,1,13,55,59,1,59,13,63,1,13,63,67,1,6,67,71,1,71,13,75,2,10,75,79,1,13,79,83,1,83,10,87,2,9,87,91,1,6,91,95,1,9,95,99,2,99,10,103,1,103,5,107,2,6,107,111,1,111,6,115,1,9,115,119,1,9,119,123,2,10,123,127,1,127,5,131,2,6,131,135,1,135,5,139,1,9,139,143,2,143,13,147,1,9,147,151,1,151,2,155,1,9,155,0,99,2,0,14,0");

    let result = execute(input);

    println!("The value at position 0 is {}", result[0]);
}

fn execute(program: String) -> Vec<String> {
    let mut p: Vec<usize> = program.split(",").map(|i| i.parse::<usize>().unwrap()).collect();
    let mut index: usize = 0;

    while p[index] != END {
        let op = Operation {
            opcode: p[index],
            source1: p[index+1],
            source2: p[index+2],
            target: p[index+3],
        };

        match op.opcode {
            o if o == ADDITION => p[op.target] = p[op.source1] + p[op.source2],
            o if o == MULTIPLICATION => p[op.target] = p[op.source1] * p[op.source2],
            _ => panic!("Bad index found"),
        };

        index += 4;
    }

    return p.iter().map(|n| n.to_string()).collect::<Vec<String>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_a_single_operation_addition_program() {
        assert_eq!(execute(String::from("1,0,0,3,99")), vec!["1","0","0","2","99"]);
    }

    #[test]
    fn it_runs_a_single_operation_multiplication_program() {
        assert_eq!(execute(String::from("2,0,0,3,99")), vec!("2","0","0","4","99"));
    }

    #[test]
    fn it_runs_a_multiple_operation_program() {
        assert_eq!(execute(String::from("1,0,0,3,2,4,4,7,99")), vec!("1","0","0","2","2","4","4","4","99"));
    }

    #[test]
    fn it_runs_a_multiple_operation_program_that_updates_the_same_index_twice() {
        assert_eq!(execute(String::from("1,0,0,0,2,4,4,0,99")), vec!("4","0","0","0","2","4","4","0","99"));
    }
}