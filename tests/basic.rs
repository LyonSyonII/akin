#[cfg(test)]
use akin::akin;

#[test]
fn basic() {
    let mut res = 0;
    akin! {
        let &var = [1, 2, 3, 4, 5];
        res += *var;
    }
    assert_eq!(res, 15)
}

#[test]
fn list() {
    let mut res = String::new();
    akin! {
        let &var = [ ['a', 'b'], ['c', 'd'], ['e'] ];
        res.extend(*var);
    }
    assert_eq!(res, "abcde")
}

#[test]
fn _match() {
    let test = |val| {
        akin! {
            let &var = ["a", "b", "c", "d"];
            let &branch = {
                *var => *var.to_owned(),
            };
            match val {
                *branch
                _ => String::new()
            }
        }
    };

    assert_eq!(test("a"), "a".to_owned());
    assert_eq!(test("b"), "b".to_owned());
    assert_eq!(test("c"), "c".to_owned());
    assert_eq!(test("d"), "d".to_owned());
}

#[test]
fn multiple_same_len() {
    use std::fmt::Write;
    
    let mut res = String::new();
    akin! {
        let &a = [1, 2, 3];
        let &b = [3, 2, 1];
        writeln!(&mut res, "*a + *b = {}", *a + *b).unwrap();
    }
    
    assert_eq!(res, "1 + 3 = 4\n2 + 2 = 4\n3 + 1 = 4\n")
}

#[test]
fn multiple_diff_len() {
    use std::fmt::Write;
    
    let mut res = String::new();
    akin! {
        let &a = [1, 2, 3, 4];
        let &b = [3, 2];
        writeln!(&mut res, "*a + *b = {}", *a + *b).unwrap();
    }
    
    assert_eq!(res, "1 + 3 = 4\n2 + 2 = 4\n3 + 2 = 5\n4 + 2 = 6\n")
}
