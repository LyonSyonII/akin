use std::mem::take;

use proc_macro::{Delimiter, Spacing, TokenTree};

#[proc_macro]
/// Duplicates the given code and substitutes specific identifiers for different code snippets in each duplicate.
///
/// ## Usage
/// Write each identifier following `let &ident = [v1, v2, v3, ...]`,
/// and use them in the snippet you want to duplicate with `*ident`.
///
/// Code snippets are copied `max(used_vars.values)` times.
/// ```
/// # use akin::akin;
/// akin! {
///     let &var = ['a', 'b'];
///     println!("{}", *var);
/// }

/// ```
/// Will get copied 2 times, because the variable `&var` has 2 values.
///
/// If a used variable has less values than another, the last one will be used.
/// ```
/// # use akin::akin;
/// akin! {
///     let &v1 = [c];
///     let &v2 = [a, b];
///     println!("*v1*v2");
/// }
/// ```
/// Expands to
/// ```rust
/// println!("ca");
/// println!("cb");
/// ```
/// All code in variables must be enclosed in brackets `{...}`.
/// ```
/// # use akin::akin;
/// akin! {
///     let &var = [-1, 1];
///     let &code = [
///         {
///             println!("true");
///         },
///         {
///             println!("false");
///         }
///     ];
///     
///     if *var < 0 {
///         *code
///     }
/// }
/// ```
/// Will expand to
/// ```rust
/// if -1 < 0 {
///     println!("true");
/// }
/// if 1 < 0 {
///     println!("false")
/// }
/// ```
/// ## Example
/// ```
/// # use akin::akin;
/// akin! {
///     let &a = [1, 2, 3, 4, 5, 6];
///     let &b = [4, 5, 6];
///     let &code = {
///         println!("*a + *b = {}", *a + *b);
///     };
///     let print = true;
///     if print {
///         *code
///     }
/// }
/// ```
/// Expands to
/// ```
/// let print = true;
/// if print {
///    println!("1 + 4 = 5");
///    println!("2 + 5 = 7");
///    println!("3 + 6 = 9");
///    println!("4 + 6 = 10");
///    println!("5 + 6 = 11");
///    println!("6 + 6 = 12");
/// }
/// ```
///
/// ## NONE
/// `NONE` is the way you can tell `akin` to simply skip that value and not write anything.  
/// It is useful for when you want to have elements in a duplication that do not have to be in the others.
/// ```
/// # use akin::akin;
/// # use std::fmt::Write;
/// # let mut out = String::new();
/// akin! {
///     let &num = [1, 2, 3];
///     let &code = [
///         NONE,
///         {
///             .pow(2)
///         }
///     ];
///
///     println!("*num^2 = {}", *num~u32*code);  
///     // *num~u32 is necessary to ensure the type is written correctly (it would be "1 u32" without it)
///     # writeln!(&mut out, "*num^2 = *numu32*code");
/// }
/// # assert_eq!(out, "1^2 = 1u32\n2^2 = 2u32 . pow( 2)\n3^2 = 3u32 . pow( 2)\n");
/// ```
///
/// ## Joint modifier
/// By default, `akin` places a space between all identifiers.  
/// Sometimes, this is not desirable, for example, if trying to interpolate between a function name
/// ```compile_fail
/// # use akin::akin;
/// akin! {
///     let &name = [1];
///     fn _*name()
///     # {}
/// }
/// ```
/// Will get wrongly expanded because '_' is an identifier
/// ```compile_fail
/// fn _ 1()
/// ```
/// To avoid it, use the joint modifier `~`, making the next identifier not to be separated.
/// ```
/// # use akin::akin;
/// akin! {  
///     let &name = [1];
///     fn _~*name() // *name is affected by the modifier
/// # {}
/// }
/// # _1();
/// ```
/// Will get correctly expanded to
/// ```
/// fn _1()
/// # {}
/// ```
/// Inside string literals `"..."` it is not necessary to use the modifier, as Rust does not count them as identifiers.
///
/// This is a limitation on proc_macro parsing, so I doubt it'll be fixed soon.
///
/// ## More examples
/// ```
/// trait Sqrt {
///     fn dumb_sqrt(self) -> Result<f64, &'static str>;
/// }
///
/// # use akin::akin;
/// akin! {
///     let &int_type = [i64, u64];
///     let &negative_check = [
///         {
///             if self < 0 {
///                 return Err("Sqrt of negative number")
///             }
///         },
///         NONE
///     ];
///     
///     let &num = [1,     2,    3,  4];
///     let &res = [1., 1.41, 1.73,  2.];
///     let &branch = {
///         *num => Ok(*res),
///     };
///
///     impl Sqrt for *int_type {
///         fn dumb_sqrt(self) -> Result<f64, &'static str> {
///             *negative_check
///             
///             match self {
///                 *branch
///                 _ => Err("Sqrt of num not in [1, 4]")
///             }
///         }
///     }
/// }
///
/// # assert_eq!(10i64.dumb_sqrt(), Err("Sqrt of num not in [1, 4]"));
/// # assert_eq!(15u64.dumb_sqrt(), Err("Sqrt of num not in [1, 4]"));
/// # assert_eq!(2u64.dumb_sqrt(), Ok(1.41));
/// # assert_eq!(3i64.dumb_sqrt(), Ok(1.73));
/// # assert_eq!((-5i64).dumb_sqrt(), Err("Sqrt of negative number"));
/// ```
/// Is expanded to:
/// ```
/// trait Sqrt {
///     fn dumb_sqrt(self) -> Result<f64, &'static str>;
/// }
///
/// impl Sqrt for i64 {
///     fn dumb_sqrt(self) -> Result<f64, &'static str> {
///         if self < 0 {
///             return Err("Sqrt of negative number")
///         }
///         
///         match self {
///             1 => Ok(1.),
///             2 => Ok(1.41),
///             3 => Ok(1.73),
///             4 => Ok(2.),
///             _ => Err("Sqrt of num not in [1, 4]")
///         }
///     }
/// }
///
/// impl Sqrt for u64 {
///     fn dumb_sqrt(self) -> Result<f64, &'static str> {
///         match self {
///             1 => Ok(1.),
///             2 => Ok(1.41),
///             3 => Ok(1.73),
///             4 => Ok(2.),
///             _ => Err("Sqrt of num not in [1, 4]")
///         }
///     }
/// }
/// ```
pub fn akin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut vars: Vec<(String, Vec<String>)> = Vec::new();
    //panic!("Tokens: {input:#?}");
    let mut tokens = input.into_iter();

    let mut first = tokens.next().expect("akin: expected code to duplicate");
    let mut second = tokens
        .next()
        .unwrap_or_else(|| panic!("akin: expected code to duplicate"));
    while matches!(&first, TokenTree::Ident(ident) if ident.to_string() == "let")
        && matches!(&second, TokenTree::Punct(punct) if punct.to_string() == "&")
    {
        vars.push(parse_var(&mut tokens, &vars));
        first = tokens
            .next()
            .unwrap_or_else(|| panic!("akin: expected code to duplicate"));
        second = tokens
            .next()
            .unwrap_or_else(|| panic!("akin: expected code to duplicate"));
    }

    let mut previous = second.clone();

    let init = fold(
        fold(String::new(), first, &mut previous),
        second,
        &mut previous,
    );
    let out_raw = tokens.fold(init, |acc, tt| fold(acc, tt, &mut previous));
    let out = duplicate(&out_raw, &vars);

    //let tokens = format!("proc_macro: {:#?}", input.into_iter().collect::<Vec<_>>());
    //let tokens = format!("vars: {:#?}", vars);
    //panic!("\nVars: {vars:#?}\nRaw: {out_raw}\nOut: {out}\n");

    out.parse().unwrap()
}

fn parse_var(
    tokens: &mut proc_macro::token_stream::IntoIter,
    vars: &[(String, Vec<String>)],
) -> (String, Vec<String>) {
    let name = format!(
        "*{}",
        tokens.next().expect("akin: expected code to duplicate")
    );
    let mut prev = tokens.next().expect("akin: expected code to duplicate"); // skip '='
    let mut values: Vec<String> = Vec::new();
    let group = tokens.next().expect("akin: expected code to duplicate");
    if let TokenTree::Group(group) = &group {
        if group.delimiter() == Delimiter::Bracket {
            let mut add = String::new();
            for var in group.stream() {
                let txt = take(&mut add);
                let new = match &var {
                    TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => 
                        g
                        .stream()
                        .into_iter()
                        .fold(String::new(), |acc, tt| fold(acc, tt, &mut prev)),
                    TokenTree::Punct(p) if p.as_char() != ',' => {
                        add = var.to_string();
                        continue;
                    }
                    _ => var.to_string(),
                };
                
                if new == "NONE" {
                    values.push(String::new())
                } else if new != "," {
                    values.push(duplicate(&(txt + &new), vars));
                }
            }
        } else {
            let fold = group
                .stream()
                .into_iter()
                .fold(String::new(), |acc, tt| fold(acc, tt, &mut prev));
            values.push(duplicate(&fold, vars));
        }

        if !matches!(tokens.next(), Some(TokenTree::Punct(p)) if p.as_char() == ';') {
            panic!("akin: expected ';' on {}'s declaration end", name.replacen('*', "&", 1));
        }
    }

    (name, values)
}

fn duplicate(stream: &str, vars: &[(String, Vec<String>)]) -> String {
    let (vars, times) = get_used_vars(stream, vars);
    let mut out = String::new();
    for i in 0..times {
        let mut temp = stream.to_owned();
        for var in &vars {
            temp = temp.replace(
                &var.0,
                var.1.get(i).unwrap_or_else(|| var.1.last().unwrap()),
            )
        }
        out += &temp;
    }
    
    if out.is_empty() {
        stream.into()
    } else {
        out
    }
}

fn get_used_vars(
    stream: &str,
    vars: &[(String, Vec<String>)],
) -> (Vec<(String, Vec<String>)>, usize) {
    let mut used = Vec::new();
    let mut times = 0;

    for var in vars {
        if stream.contains(&var.0) {
            used.push(var.clone());
            times = times.max(var.1.len());
        }
    }

    (used, times)
}

fn get_delimiters(delimiter: Delimiter) -> (char, char) {
    match delimiter {
        Delimiter::Parenthesis => ('(', ')'),
        Delimiter::Brace => ('{', '}'),
        Delimiter::Bracket => ('[', ']'),
        Delimiter::None => ('\0', '\0'),
    }
}

fn fold(a: String, tt: TokenTree, prev: &mut TokenTree) -> String {
    let ret = if let TokenTree::Group(group) = &tt {
        let (start, end) = get_delimiters(group.delimiter());
        let group = group
            .stream()
            .into_iter()
            .fold(String::new(), |acc, tt| fold(acc, tt, prev));
        format!("{a}{start}{group}{end}")
    } else if matches!(&tt, TokenTree::Punct(p) if p.as_char() == '~') {
        a // skip character
    } else if let TokenTree::Punct(p) = prev {
        // Case '*' => To make variable formatting easier ('*var' instead of '* var')
        // Case '~' => Behaviour of the '~' modifier
        if p.spacing() == Spacing::Joint || matches!(p.as_char(), '*' | '~') {
            format!("{a}{tt}")
        } else {
            format!("{a} {tt}")
        }
    } else {
        format!("{a} {tt}")
    };

    *prev = tt;
    ret
}
