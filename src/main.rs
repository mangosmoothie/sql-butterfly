use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let tokens = split_query(&input);
    let out = make_lines(tokens);
    let mut longest = 0;
    for i in (1..out.len()).step_by(2) {
        let l = match out.get(i) {
            Some(s) => s.len(),
            None => 0
        };
        if l > longest {
            longest = l;
        }
    }
    let mut iter2 = out.iter();
    while let Some(left) = iter2.next() {
        let right = iter2.next().unwrap();
        println!("{} {}",
                 format!("{: >width$}", left, width=longest + 1),
                 right);
    }

    Ok(())
}

fn make_lines(tokens: Vec<&str>) -> Vec<String> {
    let keywords = make_keyword_set();
    let mut iter = tokens.iter();
    let mut lbuffer: Vec<&str> = Vec::new();
    let mut rbuffer: Vec<&str> = Vec::new();
    let mut out: Vec<String> = Vec::new();

    while let Some(s) = iter.next() {
        if lbuffer.is_empty() {
            lbuffer.push(s);
            continue;
        }
        if keywords.contains(s.to_lowercase().as_str()) {
            out.push(lbuffer.join(" "));
            out.push(rbuffer.join(" "));
            lbuffer = match s {
                &"inner" | &"outer" => {
                    let s2 = iter.next().unwrap();
                    vec![s, s2]
                }
                &"left" | &"right" => {
                    match iter.next() {
                        Some(s2 @ &"outer") => {
                            vec![s, s2, iter.next().unwrap()]
                        },
                        Some(s2) => vec![s, s2],
                        None => Vec::new()
                    }
                }
                _ => vec![s]
            };
            rbuffer = Vec::new();
        } else {
            rbuffer.push(s);
        }
    };
    out.push(lbuffer.join(" "));
    out.push(rbuffer.join(" "));
    out
}

fn make_keyword_set() -> HashSet<&'static str> {
    ["select",
     "from",
     "where",
     "left",
     "right",
     "inner",
     "outer",
     "join",
     ",",
    ].iter().cloned().collect()
}

fn split_query(query: &str) -> Vec<&str> {
    query.split_ascii_whitespace().flat_map(|s| split_commas(s)).collect()
}

fn split_commas(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in s.match_indices(",") {
        if last != index {
            result.push(&s[last..index]);
            last = index + 1;
        }
        result.push(matched);
    }
    if last < s.len() {
        result.push(&s[last..])
    }
    result
}

#[test]
fn test_split_query() {
    let query = "SELECT * \n FROM table";
    let result = split_query(query);
    assert_eq!(vec!["SELECT", "*", "FROM", "table"], result);

    let query2 = "SELECT a, b, c FROM table";
    let result2 = split_query(query2);
    assert_eq!(vec!["SELECT", "a", ",", "b", ",", "c", "FROM", "table"], result2);
}

#[test]
fn test_split_commas() {
    let query = "a,";
    let result = split_commas(query);
    assert_eq!(vec!["a", ","], result);

    let query2 = "a,b,c";
    let result2 = split_commas(query2);
    assert_eq!(vec!["a", ",", "b", ",", "c"], result2);
}

#[test]
fn test_make_lines() {
    let input: Vec<&str> = vec!["select", "*", "FROM", "a"];
    let result = make_lines(input);
    assert_eq!(vec!["select", "*", "FROM", "a"], result);

    let input2: Vec<&str> =
        vec!["select", "a", ",", "b", "as", "bbb", "from", "t1", "inner",
             "join", "t2", "on", "t1.id", "=", "t2.id", "where", "t1.id", "=", "1"];
    let expected2: Vec<&str> =
        vec!["select", "a", ",", "b as bbb", "from", "t1", "inner join",
             "t2 on t1.id = t2.id", "where", "t1.id = 1"];
    let result2 = make_lines(input2);
    assert_eq!(expected2, result2);
}
