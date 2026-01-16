mod data;
mod routines;

pub use routines::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        ltini();
        let res = bget(1000., 50., 80.);
        dbg!(res);
        ltexi();
    }
}
