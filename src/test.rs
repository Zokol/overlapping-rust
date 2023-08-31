use super::*;

#[test]
fn integration_test() {
    let word_list = vec!["apple", "pleasure", "rendered", "clap", "suren"];
    let expected = "clappleasurendered";
    assert_eq!(print_compressed(&compress(&word_list)), expected);
}

#[test]
fn test_find_overlap() {
    assert_eq!(find_overlap("apple", "apple"), None);
    assert_eq!(find_overlap("pleasure", "apple"), None);
    assert_eq!(find_overlap("apple", "pleasure"), Some(2..5));
    assert_eq!(find_overlap("rendered", "clap"), None);
    assert_eq!(find_overlap("clap", "rendered"), None);
    assert_eq!(find_overlap("suren", "pleasure"), None);
    assert_eq!(find_overlap("pleasure", "suren"), Some(4..8));
    assert_eq!(find_overlap("apple", "rendered"), None);
    assert_eq!(find_overlap("rendered", "apple"), None);
    assert_eq!(find_overlap("clap", "apple"), Some(2..4));
}