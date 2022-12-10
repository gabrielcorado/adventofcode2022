use common;

fn main() {
    let lines = common::read_lines_vec("./day10/src/input.txt");
    println!("First part result: {}", first_part(lines.clone()));
    println!("Second part result:\n{}", second_part(lines.clone()));
}

fn first_part(lines: Vec<String>) -> isize {
    let mut x = 1;
    let mut total = 0;
    let mut cycles = 1;
    let mut next = 20;

    for line in lines {
        let (advance_by, new_x) = match line.split_once(' ') {
            Some((_, n)) => (2, x + n.parse::<isize>().unwrap()),
            None => (1, x),
        };

        if cycles + advance_by > next {
            total += next * x;
            next += 40;
        } else if cycles + advance_by == next {
            total += next * new_x;
            next += 40;
        }

        x = new_x;
        cycles += advance_by;
    }

    total
}

fn second_part(lines: Vec<String>) -> String {
    // Screen
    let mut drawn: Vec<Vec<String>> = (0..6)
        .map(|_| (0..40).map(|_| String::from(".")).collect())
        .collect();

    // Map instructions.
    let mut instructions = lines
        .iter()
        .map(|l| {
            if let Some((_, n)) = l.split_once(' ') {
                (2, n.parse::<isize>().unwrap())
            } else {
                (1, 0)
            }
        })
        .collect::<Vec<(isize, isize)>>()
        .into_iter();

    let mut reg: isize = 1;
    let mut inst: (isize, isize) = instructions.next().unwrap();
    let mut exec = inst.0;

    for x in 0..drawn.len() {
        for y in 0..drawn[0].len() {
            exec -= 1;

            // Is it in range of the sprite?
            if y + 1 >= reg as usize && y + 1 <= (reg + 2) as usize {
                drawn[x][y] = "#".to_owned();
            }

            // Execute the instruction.
            if exec == 0 {
                reg += inst.1;

                // Assign next instruction.
                if let Some(next) = instructions.next() {
                    exec = next.0;
                    inst = next;
                }
            }
        }
    }

    drawn
        .iter()
        .map(|l| l.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt")),
            13140
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            second_part(common::read_lines_vec("./src/test-input.txt")),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
