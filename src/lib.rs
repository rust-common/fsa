use std::collections::HashMap;

pub trait Node : Copy {
    fn parse(self, next: char) -> Self;
    fn id(self) -> String;
}

struct StaticNode<'t> {
    map: HashMap<char, &'t StaticNode<'t>>,
    id: &'t str
}

impl<'t> Node for &'t StaticNode<'t> {
    fn parse(self, next: char) -> Self {
        self.map[&next]
    }

    fn id(self) -> String {
        String::from(self.id)
    }
}

fn parse<T>(mut node: T, s: &str) -> T where T : Node {
    for c in s.chars() {
        node = node.parse(c);
    }
    node
}

fn parse(mut: T, s: &str) -> T where T : Node {
    for c in s.chars() {
        node = node.parse(c);
    }
    node
}


struct LambdaNode<'t> {
    transfer: &'t Fn(char, &'t Self) -> &'t Self,
    id: &'t str
}

impl<'t> LambdaNode<'t> {
    fn new(transfer: &'t Fn(char, &'t Self) -> &'t Self, id: &'t str) -> Self {
        LambdaNode {
            transfer: transfer,
            id: id
        }
    }
}

impl<'t> Node for &'t LambdaNode<'t> {
    fn parse(self, next: char) -> Self {
        (self.transfer)(next, self)
    }

    fn id(self) -> String {
        String::from(self.id)
    }
}

type TransferFn<'a> = Fn(char) -> &'a str;
type TransferMap<'a> = HashMap<&'a str, &'a TransferFn<'a>>;

fn parseMap<'a>(mut state: &'a str, t: TransferMap, s: &'a str) -> &'a str {
    for c in s.chars() {
        state = (t.get(state).unwrap())(c);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = StaticNode { map: HashMap::new(), id: "1" };
        let b = StaticNode { map: HashMap::new(), id: "2" };
        let c = StaticNode { map: HashMap::new(), id: "1" };
        assert_ne!(a.id, b.id);
        assert_eq!(a.id, c.id);
    }

    #[test]
    fn test_parse() {
        let example = "00110110";
        let error = LambdaNode::new(& |_, this| this, "error");

        let det = |c, this| {
            match c {
                '0' => this,
                '1' => this,
                _ => &error
            }
        };

        let binary = LambdaNode::new(&det, "binary");

        let result = parse(&binary, example);
        assert_eq!(result.id(), "binary");
    }

    #[test]
    fn test_parse_fail() {
        let example = "00110b110";
        let mut transfer: TransferMap = HashMap::new();
        transfer.insert("error", &|_| "error");
        transfer.insert("binary", &|c| match c {
            '0' => "binary",
            '1' => "binary",
            _ => "error"
        });

        let result = parseMap(&transfer, example);
        assert_eq!(result.id(), "error");
    }
}
