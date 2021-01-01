const MOD: u64 = 20201227;

fn get_loop_size(pk: u64) -> u64 {
    let mut value = 1u64;
    let mut loop_size = 0;
    while value != pk {
        loop_size += 1;
        value = (value * 7) % MOD;
    }
    loop_size
}

fn get_enc_key(sn: u64, ls: u64) -> u64 {
    let mut value = 1u64;
    for _ in 0..ls {
        value = (value * sn) % MOD;
    }
    value
}

fn part1(pk1: u64, pk2: u64) -> u64 {
    let ls1 = dbg!(get_loop_size(pk1));
    // dbg!(get_loop_size(pk2));

    get_enc_key(pk2, ls1)
}

fn main() {
    // full input:
    dbg!(part1(10441485, 1004920));

    // test input:
    // dbg!(part1(5764801, 17807724));
}
