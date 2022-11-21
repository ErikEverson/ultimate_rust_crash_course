// global constants not sure if this is really a great practice?
static STARTING_MISSILES: i32 = 8;
static READY_AMOUNT: i32 = 2;

fn main() {
    //extra
    // let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    // part 1
    //let mut missiles: i32 = STARTING_MISSILES;
    let missiles: i32 = STARTING_MISSILES;
    READY_AMOUNT = 365;
    let ready: i32 = READY_AMOUNT;
    // extra
    let _no_use = 365;

    println!("Firing {} of my {} missiles...", ready, missiles);

    // part 2
    //missiles = missiles - ready;
    println!("{} Missiles remainging", missiles - ready);
    // extra
    println!("{} Missiles remainging", missiles - ready - ready);
}
