pub type ParseFn<'a, Grammar> = &'a Fn(Grammar, char) -> &'a Node<'a, Grammar>;

pub struct Node<'a, Grammar> {
    consume_function: ParseFn<'a, Grammar>,
    pub name: &'static str
}
impl<'a, Grammar> Node<'a, Grammar> {
    fn new(name: &'static str, func: ParseFn<'a, Grammar>) -> Self {
        Node {
            name,
            consume_function: func
        }
    }
}

pub struct DigitGrammar<'a> {
    digit_node: Node<'a, &'a Self>,
    error_node: Node<'a, &'a Self>
}

impl<'a> Default for DigitGrammar<'a> {
    fn default() -> Self {
        Self {
            digit_node: Node::new("digit", &|grammar, c| {
                    match c {
                        '0' => &grammar.digit_node,
                        '1' => &grammar.digit_node,
                        _ => &grammar.error_node
                    }
                },
            ),
            error_node: Node::new("error", &|grammar, _| { &grammar.error_node })
        }
    }
}

impl<'a> DigitGrammar<'a> {
    /// Parse a string of characters into the grammar
    /// Example:
    /// ```
    /// use fsa::DigitGrammar;
    /// 
    /// let example = "00110110";
    /// let grammar = DigitGrammar::new();
    /// let result = grammar.parse(example);
    /// assert_eq!(result.name, "digit");
    /// ```
    pub fn parse(&self, s: &str) -> &Node<'a, &Self> {
        let mut result = &self.digit_node;
        for c in s.chars() {
            result = (result.consume_function)(self, c);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let example = "00110110";
        let grammar: DigitGrammar = Default::default();
        let result = grammar.parse(example);
        assert_eq!(result.name, "digit");
    }

    #[test]
    fn test_parse_fail() {
        let example = "00110d110";
        let grammar: DigitGrammar = Default::default();
        let result = grammar.parse(example);
        assert_eq!(result.name, "error");
    }
}
