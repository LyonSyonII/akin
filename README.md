# akin
Write repetitive code faster with akin!
Transform your long matches into much easier to manage statements

```rust
trait Sqrt {
    fn dumb_square_root(self) -> Result<f64, &str>;
}

impl Sqrt for i64 {
    fn dumb_sqrt(self) -> Result<f64, &str> {
        if num < 0 {
            return Err("Sqrt of negative number")
        }

        match num {
            1 => Ok(1),
            2 => Ok(1.41),
            3 => Ok(1.73),
            4 => Ok(2),
            _ => Err("Sqrt of num not in [1, 4]")
        }
    }
}

impl Sqrt for u64 {
    fn dumb_sqrt(self) -> Result<f64, &str> {
        match num {
            1 => Ok(1),
            2 => Ok(1.41),
            3 => Ok(1.73),
            4 => Ok(2),
            _ => Err("Sqrt of num not in [1, 4]")
        }
    }
}
```

Turns into

```rust
trait Sqrt {
    fn dumb_sqrt(self) -> Result<f64, &str>;
}

akin! {
    let &int_type = [i64, u64];
    let &negative_check = [
        if num < 0 {
            return Err("Sqrt of negative number")
        }, 
        //
    ]

    let &num = [1,    2,    3, 4];
    let &res = [1, 1.41, 1.73, 2];
    let &branches = {
        *num => Ok(*res),
    }

    impl Sqrt for *int_type {
        fn dumb_sqrt(self) -> Result<f64, &str> {
            *negative_check

            match self {
                *branches
                _ => Err("Sqrt of num not in [1, 4]")
            }
        }
    }
}
```