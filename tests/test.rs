use dog_recommendations::*;

#[test]
fn test_parse_range() {
    let input = "1-2";
    let output = parse_range(input);
    assert_eq!(output, 1..3);
    assert!(!output.contains(&0));
    assert!(output.contains(&1));
    assert!(output.contains(&2));
    assert!(!output.contains(&3));
    assert_eq!(output.start, 1);
    assert_eq!(output.end, 3);
}
