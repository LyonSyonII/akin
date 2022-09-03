use std::fmt::Write;

use proc_macro::{Delimiter, Spacing, TokenTree};

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
/// ## Range syntax
/// As a matter of convenience, the range syntax is also accepted, when declaring a variable,
/// e.g. `0..3` and `0..=3`, which are equivalent to `[0,1,2]` and `[0,1,2,3]` respectively.
/// So the variables `a` and `b` in the example above could also be declared like
///
/// ```ignore
/// let &a = 1..=6;
/// let &b = 4..=6;
/// ```
///
/// Presently, only unsigned integers that can fit in `u64` are supported in ranges, i.e. ranges
/// like `-10..-1` or `'a'..'c'`, which are fine in regular Rust, aren't accepted by `akin`.
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
#[proc_macro]
pub fn akin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut vars: Map<String, Vec<String>> = Map::new();
    //panic!("Tokens: {input:#?}");
    let mut tokens: Lookahead = input.into_iter().into();

    while let Some((name, values)) = parse_var(&mut tokens, &vars) {
        vars.insert(name, values);
    }

    let mut prev = None;
    let mut out_raw = String::new();
    for tt in tokens {
        fold_tt(&mut out_raw, tt, &mut prev);
    }

    let out = duplicate(&out_raw, &vars);

    //let tokens = format!("proc_macro: {:#?}", input.into_iter().collect::<Vec<_>>());
    //let tokens = format!("vars: {:#?}", vars);
    //panic!("\nVars: {vars:#?}\nRaw: {out_raw}\nOut: {out}\n");

    out.parse().unwrap()
}

struct Lookahead {
    queue: [Option<TokenTree>; 2],
    iter: proc_macro::token_stream::IntoIter,
}

impl Lookahead {
    fn queue_pop(&mut self) -> Option<TokenTree> {
        if let Some(tt) = self.queue[0].take() {
            self.queue[0] = self.queue[1].take();
            return Some(tt);
        }
        None
    }

    fn queue_push(&mut self, elem: TokenTree) {
        if self.queue[0].is_none() {
            self.queue[0] = Some(elem)
        } else if self.queue[1].is_none() {
            self.queue[1] = Some(elem)
        } else {
            panic!("akin: internal bug, lookahead buffer size exceeded")
        }
    }

    fn peek_nth(&mut self, i: usize) -> Option<&TokenTree> {
        assert!(i < self.queue.len(), "akin: internal bug, lookahead buffer index too big: {}", i);
        while self.queue[i].is_none() {
            if let Some(elem) = self.iter.next() {
                self.queue_push(elem);
            } else {
                return None;
            }
        }
        self.queue[i].as_ref()
    }
}

impl Iterator for Lookahead {
    type Item = <proc_macro::token_stream::IntoIter as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue_pop().or_else(|| self.iter.next())
    }
}

impl From<proc_macro::token_stream::IntoIter> for Lookahead {
    fn from(iter: proc_macro::token_stream::IntoIter) -> Self {
        Lookahead { queue: Default::default(), iter }
    }
}

fn parse_var(
    tokens: &mut Lookahead,
    vars: &Map<String, Vec<String>>,
) -> Option<(String, Vec<String>)> {
    if !matches!(tokens.peek_nth(0), Some(TokenTree::Ident(id)) if id.to_string() == "let") {
        return None;
    }

    if !matches!(tokens.peek_nth(1), Some(TokenTree::Punct(p)) if p.as_char() == '&') {
        return None;
    }

    tokens.next();
    tokens.next();

    let name = format!(
        "*{}",
        tokens.next().expect("akin: expected variable name after 'let &'")
    );

    if !matches!(tokens.next(), Some(TokenTree::Punct(p)) if p.as_char() == '=') {
        panic!( "akin: expected '=' after variable name '&{}'", &name[1..]);
    }

    let group = match tokens.next() {
        Some(TokenTree::Group(g)) => g,
        Some(l @ TokenTree::Literal(_)) => {
            tokens.queue_push(l);
            let values = parse_range_expr(&name[1..], tokens);
            return Some((name, values));
        },
        tt => panic!("akin: expected bracketed/braced group or range expression after '&{}=', got {:?}", &name[1..], tt),
    };

    let mut values: Vec<String> = Vec::new();

    if group.delimiter() == Delimiter::Bracket {
        let mut stream = group.stream().into_iter();

        while let Some(mut var) = stream.next() {
            let mut new = String::new();
            while !matches!(&var, TokenTree::Punct(p) if p.as_char() == ',') {
                match &var {
                    TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                        let mut prev = None;
                        for tt in g.stream() {
                            fold_tt(&mut new, tt, &mut prev)
                        }
                    },
                    _ => write!(&mut new, "{var}").unwrap(),
                };

                if let Some(v) = stream.next() {
                    var = v;
                } else {
                    break;
                }
            }

            if new == "NONE" {
                values.push(String::new())
            } else {
                values.push(duplicate(&new, vars));
            }
        }
    } else {
        let mut fold = String::new();
        let mut prev = None;
        for tt in group.stream() {
            fold_tt(&mut fold, tt, &mut prev)
        }
        values.push(duplicate(&fold, vars));
    }

    if !matches!(tokens.next(), Some(TokenTree::Punct(p)) if p.as_char() == ';') {
        panic!( "akin: expected ';' on end of '&{}' declaration", &name[1..]);
    }

    Some((name, values))
}

fn parse_integer_literal(tokens: &mut Lookahead) -> Result<u64, &'static str> {
    match tokens.peek_nth(0) {
        Some(TokenTree::Literal(l)) => {
            l.to_string().parse().map(|i| {
                tokens.next();
                i
            }).map_err(|_| {
                "non-integer literal"
            })
        },
        Some(_) => Err("non-literal token"),
        None => Err("unexpected end of input"),
    }
}

fn parse_range_expr(
    var_name: &str,
    tokens: &mut Lookahead,
) -> Vec<String> {
    let range_start = match parse_integer_literal(tokens) {
        Ok(v) => v,
        Err(e) => {
            panic!(
                "akin: integer literal expected after 'let &{}='{}",
                var_name, tokens.peek_nth(0).map(|tt| format!(", got {} '{}'", e, tt)).unwrap_or_default()
            );
        }
    };

    let inclusive = match (tokens.next(), tokens.next(), tokens.peek_nth(0)) {
        (Some(TokenTree::Punct(p1)), Some(TokenTree::Punct(p2)), p3) if p1.spacing() == Spacing::Joint && (p1.as_char(), p2.as_char()) == ('.', '.') => {
            p2.spacing() == Spacing::Joint && matches!(p3, Some(TokenTree::Punct(p3)) if p3.as_char() == '=')
        },
        _ => {
            panic!( "akin: expected '..' or '..=' after 'let &{}={}'", var_name, range_start);
        },
    };

    if inclusive {
        tokens.next(); // drop the '=' in '..='
    }

    let range_end = match parse_integer_literal(tokens) {
        Ok(v) => v,
        Err(e) => {
            panic!(
                "akin: integer literal expected after 'let &{}={}..'{}",
                var_name, range_start, tokens.peek_nth(0).map(|tt| format!(", got {} '{}'", e, tt)).unwrap_or_default()
            );
        }
    };

    if !matches!(tokens.next(), Some(TokenTree::Punct(p)) if p.as_char() == ';') {
        panic!( "akin: expected ';' on end of '&{}' declaration", var_name);
    }

    let last = Some(range_end).filter(|_| inclusive);
    let iter = (range_start..range_end).chain(last).map(|i| i.to_string());
    iter.collect()
}

fn duplicate(stream: &str, vars: &Map<String, Vec<String>>) -> String {
    let chunks = Chunk::new(stream).split_by_vars(vars);

    let times = chunks.iter().map(|c| c.times()).max().unwrap_or(1).max(1);

    let total_len = chunks.iter().map(|c| c.total_len(times)).sum();

    let mut out = String::with_capacity(total_len);

    for i in 0..times {
        for chunk in &chunks {
            chunk.push_to_string(i, &mut out);
        }
    }

    out
}

/// Represents a substitution chunk. A fixed piece of text followed by 0 or more text variants.
struct Chunk<'c> {
    prefix: &'c str,
    suffix_variants: &'c [String],
}

impl<'c> Chunk<'c> {
    /// Creates a chunk from a fixed piece of text.
    fn new(prefix: &'c str) -> Self {
        Chunk { prefix, suffix_variants: &[] }
    }

    fn push_to_string(&self, i: usize, out: &mut String) {
        let Chunk { prefix, suffix_variants } = *self;
        out.push_str(prefix);
        if let Some(suffix) = suffix_variants.get(i).or_else(|| suffix_variants.last()) {
            out.push_str(suffix);
        }
    }

    fn times(&self) -> usize {
        self.suffix_variants.len()
    }

    // Calculates the length of a string, that could hold `times` repetitions of this chunk.
    fn total_len(&self, times: usize) -> usize {
        let Chunk { prefix, suffix_variants } = *self;
        let mut total_len = prefix.len() * times;
        if let Some(last) = suffix_variants.last() {
            total_len += suffix_variants.iter().map(|s| s.len()).sum::<usize>();
            total_len += last.len() * times.saturating_sub(suffix_variants.len());
        }
        total_len
    }

    fn split_by_var<'s: 'c>(
        &self,
        var_name: &'s str,
        var_values: &'s [String],
    ) -> impl Iterator<Item = Chunk<'c>> {
        let Chunk { prefix, suffix_variants } = *self;

        let mut text_start = 0usize;
        let chopped = prefix.match_indices(var_name).map(move |(idx, v)| (idx, v.len(), var_values));
        let chopped = chopped.chain(std::iter::once((prefix.len(), 0, suffix_variants)));
        let chopped = chopped.map(move |(var_start, var_len, values)| {
            let new_prefix = &prefix[text_start..var_start];
            text_start = var_start + var_len;
            Chunk { prefix: new_prefix, suffix_variants: values }
        });
        chopped
    }

    fn split_by_vars<'s: 'c>(
        self,
        vars: &'s Map<String, Vec<String>>,
    ) -> Vec<Chunk<'c>> {
        let mut chunks = Vec::with_capacity(16);
        chunks.push(self);

        // Iterate over vars in reverse lexicographical order,
        // so that "*foobar" has a chance to get substituted before "*foo".
        for (name, values) in vars.iter().rev() {
            // Iterate over chunks in reverse order, so that newly inserted chunks
            // don't get processed more than once for the same variable.
            for i in (0..chunks.len()).rev() {
                chunks.splice(i..=i, chunks[i].split_by_var(name, values));
            }
        }

        chunks
    }
}

fn get_delimiters(delimiter: Delimiter) -> (char, char) {
    match delimiter {
        Delimiter::Parenthesis => ('(', ')'),
        Delimiter::Brace => ('{', '}'),
        Delimiter::Bracket => ('[', ']'),
        Delimiter::None => ('\0', '\0'),
    }
}

fn fold_tt(a: &mut String, tt: TokenTree, prev: &mut Option<TokenTree>) {
    match &tt {
        TokenTree::Group(g) => {
            let (start, end) = get_delimiters(g.delimiter());
            a.push(start);
            for tt in g.stream() {
                fold_tt(a, tt, prev);
            }
            a.push(end);
        }
        TokenTree::Punct(p) if p.as_char() == '~' => {
            // skip character
        }
        _ if matches!(&prev, Some(TokenTree::Punct(p)) if p.spacing() == Spacing::Joint || matches!(p.as_char(), '*' | '~')) => {
            // Case '*' => To make variable formatting simpler ('*var' instead of '* var')
            // Case '~' => Behaviour of the '~' modifier
            write!(a, "{tt}").unwrap();
        }
        _ => {
            write!(a, " {tt}").unwrap();
        }
    };

    *prev = Some(tt);
}

type Map<T, S> = std::collections::BTreeMap<T, S>;
