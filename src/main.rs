fn main() {
    let mut a = 1;
    let mut ra = &mut a;
    let mut rra = &mut ra;
    let mut rrra: &mut &mut &mut i32 = &mut rra;
    ***rrra += 1;
    println!("{rrra}")
}
