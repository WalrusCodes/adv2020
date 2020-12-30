#[derive(Debug, Clone, Copy)]
struct Node {
    value: u32,
    next: *mut Node,
    prev: *mut Node,
}

struct State {
    // node[0] = 1, etc.
    nodes: Vec<Node>,
    current_cup: *mut Node,
}

impl State {
    fn new() -> State {
        let mut nodes = Vec::new();
        for i in 0..1_000_000 {
            nodes.push(Node {
                value: i + 1,
                next: std::ptr::null_mut(),
                prev: std::ptr::null_mut(),
            });
        }
        for i in 0..1_000_000 {
            nodes[i].next = &mut nodes[(i + 1) % 1_000_000];
            nodes[i].prev = &mut nodes[(i + 999_999) % 1_000_000];
        }
        let current_cup: *mut Node = &mut nodes[0];
        State { nodes, current_cup }
    }

    fn build(input: u32) -> State {
        let mut state = State::new();

        let mut previous = input / 100_000_000;
        let mut inp = (input % 100_000_000) * 10;

        state.current_cup = &mut state.nodes[(previous - 1) as usize];

        for _ in 0..8 {
            let value = inp / 100_000_000;
            inp = (inp % 100_000_000) * 10;
            state.move_cup(value, previous);
            previous = value;
        }

        // <=> [0] <=> [1] <=> [2] <=> [3] <=> .... [1_000_000] <=>
        //
        //                              v      8    9
        // <=> [0] <=> [1] <=> [2] <=> [3] <=> .... [1_000_000] <=>
        //
        state
    }

    fn move_cup(self: &mut Self, which_value: u32, after_value: u32) {
        // dbg!(which_value, after_value);
        let which: *mut Node = &mut self.nodes[(which_value - 1) as usize];
        let after: *mut Node = &mut self.nodes[(after_value - 1) as usize];
        // a <=> which <=> b ... after <=> y
        //
        // a <=> b  ... after <=> which <=> y
        unsafe {
            let a = (*which).prev;
            let b = (*which).next;

            (*a).next = b;
            (*b).prev = a;

            let y = (*after).next;

            (*after).next = which;
            (*y).prev = which;
            (*which).next = y;
            (*which).prev = after;
        }
    }

    fn calculate_dest_cup_value(self: &mut Self) -> u32 {
        let mut dest_cup_value = unsafe { (*self.current_cup).value - 1 };
        let next = unsafe { (*self.current_cup).next };
        let next2 = unsafe { (*next).next };
        let next3 = unsafe { (*next2).next };
        let bad_values = [
            unsafe { (*next).value },
            unsafe { (*next2).value },
            unsafe { (*next3).value },
        ];
        loop {
            if dest_cup_value == 0 {
                dest_cup_value = 1_000_000;
            } else if bad_values.contains(&dest_cup_value) {
                dest_cup_value -= 1;
            } else {
                break;
            }
        }
        dest_cup_value
    }

    fn do_move(self: &mut Self) {
        let dest_cup_value = self.calculate_dest_cup_value();
        // dbg!(&dest_cup_value);

        let next = unsafe { (*self.current_cup).next };
        let next2 = unsafe { (*next).next };
        let next3 = unsafe { (*next2).next };

        let next_value = unsafe { (*next).value };
        let next_value2 = unsafe { (*next2).value };
        let next_value3 = unsafe { (*next3).value };

        // dbg!(next_value, next_value2, next_value3);

        self.move_cup(next_value, dest_cup_value);
        self.move_cup(next_value2, next_value);
        self.move_cup(next_value3, next_value2);

        self.current_cup = unsafe { (*self.current_cup).next };

        // From: <starting><3 cups that we move><XXXXX><dest cup><YYYYY>
        //
        // To:   <XXXXX><dest cup><3 cups that we move><YYYYY><starting>
    }

    // For part 2:
    //
    // * 10'000'000 moves
    // * 1'000'000 numbers
    // * need:
    //   * quick way to find a cup by label
    //   * quick way to yank out 3 cups and insert them after destination cup
    //
    // 1: ----------------------------------------.
    // 2:                                         |
    // 3: -                                       |
    //    |                                       v
    //    v
    //  [ 3 ] <==> [ 8 ] <==> [ 9 ] <===>  ..... [ 1'000'000 ]
}

fn main() {
    // let simple_input = 389125467;
    let input = 219347865;
    let mut state = State::build(input);
    for _ in 0..10_000_000 {
        state.do_move();
    }
    unsafe {
        let next = state.nodes[0].next;
        let next2 = (*next).next;
        dbg!(*next, *next2);
        let product = ((*next).value as u64) * ((*next2).value as u64);
        dbg!(product);
    }
}
