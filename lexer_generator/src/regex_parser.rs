use std::rc::Rc;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct RegexTerminal {
    pub pos: usize,
    pub ch: char,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum RegexNode {
    Cat(Rc<RegexNode>, Rc<RegexNode>),
    Or(Rc<RegexNode>, Rc<RegexNode>),
    Parenthesized(Rc<RegexNode>),
    Kleene(Rc<RegexNode>),
    Terminal(RegexTerminal),
}

pub fn parse_regex(pattern: &str) -> Rc<RegexNode> {
    let parser = RegexParser::default();
    parser.parse(pattern)
}

#[derive(Default)]
struct RegexParser {
    current_pos: usize,
}

impl RegexParser {
    fn parse(mut self, pattern: &str) -> Rc<RegexNode> {
        match pattern {
            "+" | "*" | "(" | ")" => {
                let ch = pattern.chars().next().unwrap();
                let node = self.single_char(ch);
                self.augment(node)
            }
            "\\d\\d*" => {
                let right = self.number();
                let right = Self::kleene(right);
                let left = self.number();
                self.augment(Self::cat(left, right))
            }
            _ => unimplemented!(),
        }
    }

    fn augment(&mut self, node: Rc<RegexNode>) -> Rc<RegexNode> {
        let sentinel = Rc::new(RegexNode::terminal('\0', self.current_pos));
        self.current_pos += 1;
        Self::cat(node, sentinel)
    }

    fn number(&mut self) -> Rc<RegexNode> {
        let mut node = None;
        for i in 0..10 {
            let ch = char::from_digit(i, 10).unwrap();
            match node {
                None => node = Some(self.single_char(ch)),
                Some(n) => node = Some(Self::or(n, self.single_char(ch))),
            }
        }
        node.unwrap()
    }

    fn single_char(&mut self, ch: char) -> Rc<RegexNode> {
        let char = Rc::new(RegexNode::terminal(ch, self.current_pos));
        self.current_pos += 1;
        char
    }

    fn cat(left: Rc<RegexNode>, right: Rc<RegexNode>) -> Rc<RegexNode> {
        Rc::new(RegexNode::Cat(left, right))
    }

    fn or(left: Rc<RegexNode>, right: Rc<RegexNode>) -> Rc<RegexNode> {
        Rc::new(RegexNode::Or(left, right))
    }

    fn kleene(node: Rc<RegexNode>) -> Rc<RegexNode> {
        Rc::new(RegexNode::Kleene(node))
    }
}
