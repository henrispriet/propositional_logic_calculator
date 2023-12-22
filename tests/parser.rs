use propositional_logic_calculator::{
    expression::{not, var, Expression},
    parser::Parser,
};

#[test]
fn test_parse_simple_expression() {
    let mut parser = Parser::new("A");
    assert_eq!(parser.parse().unwrap(), Expression::Var("A".to_string()));
}

#[test]
fn test_parse_and_expression() {
    let mut parser = Parser::new("A&B");
    assert_eq!(
        parser.parse().unwrap(),
        *var("A").and(var("B"))
    );
}

#[test]
fn test_parse_or_expression() {
    let mut parser = Parser::new("A|B");
    assert_eq!(
        parser.parse().unwrap(),
        *var("A").or(var("B"))
    );
    let mut parser = Parser::new("AvB");
    assert_eq!(
        parser.parse().unwrap(),
        *var("A").or(var("B"))
    );
}

#[test]
fn test_parse_not_expression() {
    let mut parser = Parser::new("-A");
    assert_eq!(
        parser.parse().unwrap(),
        *not(var("A"))
    );
}

#[test]
fn test_parse_nested_expression() {
    let mut parser = Parser::new("-(A&B)");
    assert_eq!(
        parser.parse().unwrap(),
        *not(var("A").and(var("B")))
    );
}

#[test]
fn test_parse_with_unmatched_parentheses() {
    let mut parser = Parser::new("A&");
    assert!(parser.parse().is_err());
}

#[test]
fn test_parse_with_invalid_character() {
    let mut parser = Parser::new("A$B");
    assert!(parser.parse().is_err());
}

#[test]
fn test_ingore_invalid_characters() {
    let mut parser = Parser::new("A & B");
    assert_eq!(
        parser.parse().unwrap(),
        *var("A").and(var("B"))
    );
}

#[test]
fn test_ignore_invalid_character_in_brackets() {
    let mut parser = Parser::new("(A & B)");
    assert_eq!(
        parser.parse().unwrap(),
        *var("A").and(var("B"))
    );
}

#[test]
fn test_deeply_nested_expression() {
    let mut parser = Parser::new("(((((A))))&B)");
    assert_eq!(
        parser.parse().unwrap(),
        *var("A").and(var("B"))
    );
}

#[test]
fn test_invalid_nesting() {
    let mut parser = Parser::new("(A&B))|(C");
    assert!(parser.parse().is_err());
}

#[test]
fn test_empty_input() {
    let mut parser = Parser::new("");
    assert!(parser.parse().is_err());
}

#[test]
fn test_repeated_operators() {
    let mut parser = Parser::new("A&&B");
    assert!(parser.parse().is_err());
}

#[test]
fn test_only_operators() {
    let mut parser = Parser::new("&|>");
    assert!(parser.parse().is_err());
}

#[test]
fn test_invalid_characters() {
    let mut parser = Parser::new("A&B#C");
    assert!(parser.parse().is_err());
}
