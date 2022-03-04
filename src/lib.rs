use proc_macro::{Delimiter, Span, TokenTree};
use proc_macro_error::{proc_macro_error, abort};
extern crate proc_macro;

#[proc_macro_error]
#[proc_macro]
pub fn akin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut vars: Vec<(String, Vec<String>)> = Vec::new();
    //panic!("Tokens: {input:#?}");
    let mut tokens = input.into_iter();
    

    let mut first = tokens.next().unwrap();
    let mut second = tokens.next().unwrap();
    while matches!(&first, TokenTree::Ident(ident) if ident.to_string() == "let") && matches!(&second, TokenTree::Punct(punct) if punct.to_string() == "&") {
        vars.push(parse_var(&mut tokens, &vars));
        first = tokens.next().unwrap();
        second = tokens.next().unwrap();
    }
    
    fn fold(a: String, tt: TokenTree) -> String {
        if let TokenTree::Group(group) = tt {
            let (start, end) = get_delimiters(group.delimiter());  
            let group = group.stream().into_iter().fold(String::new(), fold);
            format!("{a}{start}{group}{end}")
        } else if a.ends_with('*') {
            format!("{a}{tt}")
        } else {
            format!("{a} {tt}")
        }
    }
    
    let init = fold(fold(String::new(), first), second);
    let out_raw = tokens.fold(init, fold);
    let out = duplicate(&out_raw, &vars);
    
    
    //let tokens = format!("proc_macro: {:#?}", input.into_iter().collect::<Vec<_>>());
    //let tokens = format!("vars: {:#?}", vars);
    
    
    
    out.parse().unwrap()
}

fn parse_var(tokens: &mut proc_macro::token_stream::IntoIter, vars: &[(String, Vec<String>)]) -> (String, Vec<String>) {
    let name = format!("*{}", tokens.next().unwrap());
    tokens.next().unwrap(); // skip '='
    let mut values: Vec<String> = Vec::new();
    let group = tokens.next().unwrap();
    if let TokenTree::Group(group) = &group {
        if group.delimiter() == Delimiter::Bracket {
            for var in group.stream() {
                let txt = var.to_string();
                if txt != "," {
                    values.push(txt);
                }
            }
        } else {
            values.push(duplicate(&group.stream().to_string(), vars));
        }
        
        if tokens.next().unwrap().to_string() != ";" {
            abort!(group.span_close(), "Expected ';'")
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
            temp = temp.replace(&var.0, var.1.get(i).unwrap_or_else(|| var.1.last().unwrap()))
        }
        out += &temp;
    }
    out
}

fn get_used_vars(stream: &str, vars: &[(String, Vec<String>)]) -> (Vec<(String, Vec<String>)>, usize) {
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