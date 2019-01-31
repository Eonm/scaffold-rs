#[derive(Debug)]
#[derive(PartialEq)]
enum PathElement {
    Path(String),
    Enumerator(String)
}

fn split_str_at_delimiters <'a> (input_str: &'a str, delimiters: &(&'a str,&'a str)) -> Vec<PathElement> {
    let mut return_string : Vec<PathElement> = vec!();
    let (start, end) = (input_str.find(delimiters.0), input_str.find(delimiters.1));
    match (start,end) {
        (Some(s), Some(e)) => {
            if s > e {
                let (first, rest) = input_str.split_at(s);
                    return_string.push(PathElement::Path(first.to_string()));
                    return_string.append(&mut split_str_at_delimiters(rest, delimiters))
            }  else {
                let (first, rest) = input_str.split_at(s);
                if !first.is_empty() {
                  return_string.push(PathElement::Path(first.to_string()));
                }
                let (beg,ed) = rest.split_at(e-first.len()+1);
                match beg.rfind(delimiters.0) {
                    Some(index) => {
                        if index != 0 {
                            let (b,e) = beg.split_at(index);
                            return_string.append(&mut split_str_at_delimiters(b, delimiters));
                            return_string.append(&mut split_str_at_delimiters(e, delimiters));
                        } else {
                            let first_delimiter = delimiters.0.chars().next().unwrap();
                            let second_delimiter = delimiters.1.chars().next().unwrap();
                            let range : Vec<&str> = beg.split(&[first_delimiter, second_delimiter][..]).collect();
                            return_string.push(PathElement::Enumerator(range[1].to_string()));
                        }
                    },
                    None => {
                    }
                }
                return_string.append(&mut split_str_at_delimiters(ed, delimiters));
            }
        },
        _ =>  if !input_str.is_empty() {return_string.push(PathElement::Path(input_str.to_string()))}
    }
    return return_string
}

fn str_to_range(r: &String) -> std::ops::Range<i32> {
    let rr : Vec<i32> = r.split("-")
        .map(|elem| elem.parse::<i32>().expect("This is not a number"))
        .collect();

        if rr[0] < rr[1] {
            return rr[0]..rr[1]+1
        } else {
            return rr[1]..rr[0]+1
        }
}

fn assemble_pairs(input_vec : Vec<PathElement>) -> Vec<String> {
    input_vec.iter().fold(vec!(),|mut accumulator, elem| {
        if accumulator.is_empty() {
            match elem {
                PathElement::Enumerator(range) => {
                    let range = str_to_range(range);
                    for r in range {
                        accumulator.push(format!("{}", r));
                    }
                 },
                 PathElement::Path(path) => {
                     accumulator.push(path.to_string());
                 }
            }
            accumulator
        } else {
            match elem {
                PathElement::Enumerator(range) => {
                    accumulator.iter().flat_map(|previous_text| {
                        let range = str_to_range(range);
                        range.map(|a| {
                            format!("{}{}",previous_text, a)
                        }).collect::<Vec<String>>()
                    }).collect::<Vec<String>>()
                },
                PathElement::Path(path) => {
                    accumulator.iter().map(|previous_text| {
                        format!("{}{}", previous_text, path)
                    }).collect::<Vec<String>>()
                }
            }
        }
    })
}

pub fn generate_paths(input_str : &str, delimiters : (&str,&str)) -> Vec<String> {
    let splited_string = split_str_at_delimiters(input_str, &delimiters);
    let paths = assemble_pairs(splited_string);
    paths
}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_str_at_delimiters () {
        let input = "./test1-2][1-2]";
        let return_vec = [PathElement::Path("./test1-2]".to_string()), PathElement::Enumerator("1-2".to_string())];
        assert_eq!(split_str_at_delimiters(input, &("[","]")), &return_vec);

        let input_2 = "[1-2]test[1-2]";
        let return_vec_2 = [PathElement::Enumerator("1-2".to_string()), PathElement::Path("test".to_string()), PathElement::Enumerator("1-2".to_string())];
        assert_eq!(split_str_at_delimiters(input_2, &("[","]")), &return_vec_2);

        let input_3 = "[1-2test[[1-2]][s";
        let return_vec_3 = [PathElement::Path("[1-2test[".to_string()),
        PathElement::Enumerator("1-2".to_string()),
        PathElement::Path("]".to_string()),
        PathElement::Path("[s".to_string())];
        assert_eq!(split_str_at_delimiters(input_3, &("[","]")), &return_vec_3);
    }

    #[test]
    fn generate_single_range() {
        assert_eq!(generate_paths("./test[1-2]", ("[","]")), ["./test1", "./test2"]);
        assert_eq!(generate_paths("[1-2]test", ("[","]")), ["1test", "2test"]);
        assert_eq!(generate_paths("e[2-5]test", ("[","]")), ["e2test", "e3test", "e4test", "e5test"]);
    }

    #[test]
    fn generate_mutliple_range() {
        assert_eq!(generate_paths("[1-2]test[1-2]", ("[","]")), ["1test1", "1test2", "2test1", "2test2"]);
    }

    #[test]
    fn only_parse_closed() {
        assert_eq!(generate_paths("./test1-2][1-2]", ("[","]")), ["./test1-2]1", "./test1-2]2"]);
        assert_eq!(generate_paths("./test1-2][1-2", ("[","]")), ["./test1-2][1-2"]);
        assert_eq!(generate_paths("./test1-21-2]", ("[","]")), ["./test1-21-2]"]);
    }

    #[test]
    fn only_parse_separator () {
        assert_eq!(generate_paths("./test(1-2)[1-2]d", ("(",")")), ["./test1[1-2]d", "./test2[1-2]d"]);
        assert_eq!(generate_paths("./test1-2](1-2)", ("(",")")), ["./test1-2]1", "./test1-2]2"]);
        assert_eq!(generate_paths("./test(1-2]-21-2]", ("(","]")), ["./test1-21-2]","./test2-21-2]"]);
    }

    #[test]
    fn test_text_ranges () {
        assert_eq!(str_to_range(&"1-5".to_string()),(1..6));
        assert_eq!(str_to_range(&"5-2".to_string()),(2..6));
    }

    #[test]
    #[should_panic]
    fn str_to_range_should_failed () {
        str_to_range(&"[1-5]".to_string());
    }
}
