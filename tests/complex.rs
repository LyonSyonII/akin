use std::str::FromStr;

#[cfg(test)]
use akin::akin;

#[test]
fn operations() {
    let (a, b) = (5, 6);
    akin! {
        let &op = [+, -, *, /];
        println!("a *op b = {}", a *op b);
    }
}
