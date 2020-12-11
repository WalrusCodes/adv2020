// Returns true if two numbers in window sum up to "sum".
fn find_sum(sum: u64, window: &[u64]) -> bool {
    for (i, num1) in window.iter().enumerate() {
        for num2 in window[i + 1..].iter() {
            if (num1 + num2) == sum {
                return true;
            }
        }
    }
    false
}

fn part1(input: &[u64]) {
    for i in 25..input.len() {
        let sum = input[i];
        if !find_sum(sum, &input[i - 25..i]) {
            dbg!(sum);
            break;
        }
    }
}

fn part2(input: &[u64]) {
    const TO_FIND: u64 = 1930745883;
    // const TO_FIND: u64 = 127;
    let mut head = 0;
    let mut tail = 0;
    let mut sum = 0;
    while sum != TO_FIND {
        if sum < TO_FIND {
            sum += input[tail];
            tail += 1;
        } else {
            sum -= input[head];
            head += 1;
        }
    }
    dbg!(sum, head, tail);
    let min = input[head..tail].iter().min().unwrap();
    let max = input[head..tail].iter().max().unwrap();
    let answer = min + max;
    dbg!(answer);
}

fn main() {
    let contents = std::fs::read_to_string("input/9.txt").expect("read failed");
    let input = contents
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    // part1(input.as_slice());
    part2(input.as_slice());
}
