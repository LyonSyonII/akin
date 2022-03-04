use proc_macro::{Delimiter, TokenTree, Punct};
use proc_macro_error::{abort, proc_macro_error};

#[proc_macro_error]
#[proc_macro]
pub fn akin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut vars: Vec<(String, Vec<String>)> = Vec::new();
    //panic!("Tokens: {input:#?}");
    let mut tokens = input.into_iter();
    
    let mut first = tokens.next().expect("akin: expected code to duplicate");
    let mut second = tokens
        .next()
        .unwrap_or_else(|| abort!(first.span(), "akin: expected code to duplicate"));
    while matches!(&first, TokenTree::Ident(ident) if ident.to_string() == "let")
        && matches!(&second, TokenTree::Punct(punct) if punct.to_string() == "&")
    {
        vars.push(parse_var(&mut tokens, &vars));
        first = tokens
            .next()
            .unwrap_or_else(|| abort!(second.span(), "akin: expected code to duplicate"));
        second = tokens
            .next()
            .unwrap_or_else(|| abort!(first.span(), "akin: expected code to duplicate"));
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
    //panic!("{tokens}");
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
            for var in group.stream() {
                let txt = if let TokenTree::Group(group) = &var {
                    if group.delimiter() == Delimiter::Brace {
                        group
                            .stream()
                            .into_iter()
                            .fold(String::new(), |acc, tt| fold(acc, tt, &mut prev))
                    } else {
                        var.to_string()
                    }
                } else {
                    var.to_string()
                };

                if txt == "NONE" {
                    values.push(String::new())
                } else if txt != "," {
                    values.push(duplicate(&txt, vars));
                }
            }
        } else {
            let fold = group
                .stream()
                .into_iter()
                .fold(String::new(), |acc, tt| fold(acc, tt, &mut prev));
            values.push(duplicate(&fold, vars));
        }

        if tokens.next().expect("akin: expected ';'").to_string() != ";" {
            abort!(group.span_close(), "akin: expected ';'")
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

    if out == String::new() {
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
    if let TokenTree::Group(group) = &tt {
        let (start, end) = get_delimiters(group.delimiter());
        let group = group
            .stream()
            .into_iter()
            .fold(String::new(), |acc, tt| fold(acc, tt, prev));
        *prev = tt;
        format!("{a}{start}{group}{end}")
    } 
    else if matches!(&tt, TokenTree::Punct(p) if p.as_char() == '#') {
        *prev = tt.clone();
        format!("{a} ")
    } 
    else if let TokenTree::Punct(p) = &prev {
        *prev = if p.as_char() == '#' {
            TokenTree::Punct(Punct::new('$', proc_macro::Spacing::Joint))
        } else {
            tt.clone()
        };

        format!("{a}{tt}")
        
    } else {
        *prev = tt.clone();
        format!("{a} {tt}")
    }
}
