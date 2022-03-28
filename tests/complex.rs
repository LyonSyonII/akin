#[cfg(test)]
use akin::akin;

#[test]
fn operations() {
    let mut res = Vec::new();
    let (a, b) = (5, 6);
    akin! {
        let &op = [+, -, *, /];
        res.push(format!("a *op b = {}", a *op b));
    }

    assert_eq!(res, ["a + b = 11", "a - b = -1", "a * b = 30", "a / b = 0"]);
}

#[test]
fn replace() {
    akin! {
        let &ints = [u64, u32];
        let &int = { *ints: *ints, };
        let &floats = [f64, f32];
        let &float = { *floats: *floats, };

        enum Dyn {
            Int,
            Float,
        }

        union Int {
            *int
        }

        union Float {
            *float
        }
    }

    let i1 = Int { u64: 5  };
    let i2 = Int { u32: 56 };
    let f1 = Float { f64: 1. };
    let f2 = Float { f32: 89.0 };

    unsafe {
        assert_eq!(i1.u64, 5);
        assert_eq!(i1.u32, 5);
        assert_eq!(i2.u64, 56);
        assert_eq!(i2.u32, 56);
        assert_eq!(f1.f64, 1.);
        assert_ne!(f1.f32, 1.);
        assert_eq!(f2.f32, 89.);
        assert_ne!(f2.f64, 89.);
    }
}

// tests for the akin! macro
