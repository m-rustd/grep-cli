use grep_cli::{search, search_case_insensitive};

#[test]
fn search_test() {
    let contents = "\
hello world
i Have a good friend
";
    let query = "world";
    assert_eq!(vec!["hello world"], search(query, contents));
}


#[test]
fn search_case_insensitive_test() {
    let contents = "\
hello world
i Have a good friend
";
    let query = "WORLD";
    assert_eq!(vec!["hello world"], search_case_insensitive(query, contents));
}