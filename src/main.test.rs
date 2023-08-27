#[test]
fn test_find_overlap() {
    assert_eq!(find_overlap("apple", "apple"), (5, 0..5));
    assert_eq!(find_overlap("pleasure", "apple"), (0, 0..0));
    assert_eq!(find_overlap("apple", "pleasure"), (3, 2..4));
    assert_eq!(find_overlap("rendered", "clap"), (0, 0..0));
    assert_eq!(find_overlap("clap", "rendered"), (0, 0..0));
    assert_eq!(find_overlap("suren", "pleasure"), (0, 0..0));
    assert_eq!(find_overlap("pleasure", "suren"), (4, 4..7));
    assert_eq!(find_overlap("apple", "rendered"), (0, 0..0));
    assert_eq!(find_overlap("rendered", "apple"), (0, 0..0));
    assert_eq!(find_overlap("clap", "apple"), (2, 2..3));
}