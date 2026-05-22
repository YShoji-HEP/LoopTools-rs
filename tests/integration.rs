#[test]
fn loop_b() {
    looptools_rs::ltini();
    let result = looptools_rs::bget(1000., 50., 80.);
    println!("{:?}", result);
    looptools_rs::ltexi();
}