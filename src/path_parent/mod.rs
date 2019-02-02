pub fn replace_with_parent_dir (path: String) -> String {
    let splited_path = path.split("/").map(|p| p.to_string()).collect::<Vec<String>>();
    let return_value : Vec<String> = splited_path.iter().fold(vec!(),|mut acc, p| {
        if p.contains("[*]") {
            match acc.clone().last() {
                Some(previous_dir) => {
                    let substitution = if acc.len() > 1 {
                        p.replace("[*]", previous_dir)
                    } else {
                        p.replace("[*]", "")
                    };
                    acc.push(substitution.to_string());
                },
                None => {
                    acc.push(p.replace("[*]", ""))
                }
            };
        } else {
            acc.push(p.to_string())
        }
        acc
    });
    return_value.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_with_parent_dir () {
        let input = "./dir1/file1";
        assert_eq!(replace_with_parent_dir(input.to_string()), input);

        let input1 = "./dir1/[*]_file1";
        assert_eq!(replace_with_parent_dir(input1.to_string()), "./dir1/dir1_file1");
    }

    #[test]
    fn test_dont_replace_if_ancestor_dir_dont_exist () {
        let input = "./[*]dir1/file1";
        assert_eq!(replace_with_parent_dir(input.to_string()), "./dir1/file1");

        let input1 = "[*]/dir1/file1";
        assert_eq!(replace_with_parent_dir(input1.to_string()), "/dir1/file1");

        let input2 = "[*]/[*]dir1/file1";
        assert_eq!(replace_with_parent_dir(input2.to_string()), "/dir1/file1");
    }

    #[test]
    fn test_replace_multiple_parent_dirs() {
        let input = "./dir/[*]_file_1_[*]";
        assert_eq!(replace_with_parent_dir(input.to_string()), "./dir/dir_file_1_dir");

        let input1 = "./dir/sub_[*]/file_[*].txt";
        assert_eq!(replace_with_parent_dir(input1.to_string()), "./dir/sub_dir/file_sub_dir.txt");
    }
}
