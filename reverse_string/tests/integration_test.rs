use reverse_string::reverse;

#[test]
fn test_reverse_integration() {
    assert_eq!(reverse("Integration"), "noitargetnI");
    assert_eq!(reverse("Test"), "tseT");
}
