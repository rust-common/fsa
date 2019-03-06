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
}
