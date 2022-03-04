# akin
Crate for writing repetitive code easier and faster.
Check [Syntax](#syntax) for information about how to use it.

# Why?
I've found myself having to write a lot of repetitive code (mostly when matching against enums in parsing).  
The fantastic [duplicate](https://crates.io/crates/duplicate) lacked the ability to write invariable code (code that didn't get duplicated) and had a too different syntax,  so I decided to make my own tool.  


# Example
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
    let &branch = {
        *num => Ok(*res),
    }

    impl Sqrt for *int_type {
        fn dumb_sqrt(self) -> Result<f64, &str> {
            *negative_check

            match self {
                *branch
                _ => Err("Sqrt of num not in [1, 4]")
            }
        }
    }
}
```

Is expanded to:

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

The good thing about **akin** is that it detects automatically the number of values of each variable *for each scope*, so for example "branch" will get copied 4 times (as "num" and "res" both have 4 values), but the main function will only be duplicated once, as all the variables it has have 2 values.

# Syntax
The crate only provides one macro, `akin!`.
The syntax is as follows:

First, you declare the variables you'll use. 
A variable name must start with `&`, as it's the only way the macro can differentiate between macro or real declaratinons.

```rust
let &variable = [v1, v2, v3, ...]
let &variable2 = [...]
    
let &code = {
    ...
}
```

Then, when all variables have been declared, you can write the code snippet you want to duplicate.  
The amount of times it will be duplicated depends on the variables that are used.