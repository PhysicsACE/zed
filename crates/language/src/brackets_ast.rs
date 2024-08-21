pub enum AstNodeKind {
    Text,
    Bracket,
    Pair,
    UnexpectedClosingBracket,
    List,
}

pub enum AstNode<'a> {
    Text(TextAstNode),
    Bracket(BracketAstNode),
    Pair(PairAstNode<'a>),
    UnexpectedClosingBracket(InvalidBracketAstNode),
    List(ListAstNode<'a>),
}

impl<'a> AstNode<'a> {
    pub fn kind(&self) -> AstNodeKind {
        match self {
            AstNode::Text(_) => AstNodeKind::Text,
            AstNode::Bracket(_) => AstNodeKind::Bracket,
            AstNode::Pair(_) => AstNodeKind::Pair,
            AstNode::UnexpectedClosingBracket(_) => AstNodeKind::UnexpectedClosingBracket,
            AstNode::List(_) => AstNodeKind::List,
        }
    }

    pub fn children_length(&self) -> usize {
        match self {
            AstNode::Pair(pair) => pair.children_length(),
            AstNode::List(list) => list.children_length(),
            _ => 0,
        }
    }

    pub fn get_child(&self, index: usize) -> Option<&AstNode<'a>> {
        match self {
            AstNode::Pair(pair) => pair.get_child(index),
            AstNode::List(list) => list.get_child(index),
            _ => None,
        }
    }

    pub fn children(&self) -> Vec<&AstNode<'a>> {
        match self {
            AstNode::Pair(pair) => pair.children(),
            AstNode::List(list) => list.children(),
            _ => Vec::new(),
        }
    }

    pub fn node_height(&self) -> usize {
        match self {
            AstNode::Pair(pair) => pair.node_height(),
            AstNode::List(list) => list.node_height(),
            _ => 0,
        }
    }

    pub fn length(&self) -> u64 {
        match self {
            AstNode::Pair(pair) => pair.length,
            AstNode::List(list) => list.length,
            AstNode::Text(text) => text.length,
            AstNode::Bracket(bracket) => bracket.length,
            AstNode::UnexpectedClosingBracket(invalid) => invalid.length,
        }
    }

    pub fn list_height(&self) -> usize {
        match self {
            AstNode::List(list) => list.node_height(),
            _ => 0,
        }
    }

    pub fn deepclone(&self) -> AstNode<'a> {
        match self {
            AstNode::Text(text) => AstNode::Text(text.deepclone()),
            AstNode::Bracket(bracket) => AstNode::Bracket(bracket.deepclone()),
            AstNode::Pair(pair) => AstNode::Pair(pair.deepclone()),
            AstNode::UnexpectedClosingBracket(invalid) => {
                AstNode::UnexpectedClosingBracket(invalid.deepclone())
            }
            AstNode::List(list) => AstNode::List(list.deepclone()),
        }
    }
}

#[derive(Clone)]
pub struct PairAstNode<'a> {
    pub length: u64,
    pub opening_bracket: BracketAstNode,
    pub closing_bracket: BracketAstNode,
    pub child: Option<&'a AstNode<'a>>,
}

impl<'a> PairAstNode<'a> {
    pub fn new(
        opening_bracket: BracketAstNode,
        closing_bracket: BracketAstNode,
        child: Option<&'a AstNode<'a>>,
    ) => PairAstNode {
        let mut length = opening_bracket.length;
        if let Some(child) = child {
            length += child.length();
        }
        length += closing_bracket.length;
        PairAstNode {
            length,
            opening_bracket,
            closing_bracket,
            child,
        }
    }

    pub fn children_length(&self) -> usize {
        if let Some(child) = self.child {
            2 + child.children_length()
        } else {
            2
        }
    }

    pub fn get_child(&self, index: usize) -> Option<&AstNode<'a>> {
        if index == 0 {
            Some(&self.opening_bracket)
        } else if index == 1 {
            self.child
        } else if index == 2 {
            Some(&self.closing_bracket)
        } else {
            None
        }
    }

    pub fn children(&self) -> Vec<&AstNode<'a>> {
        if let Some(child) = self.child {
            vec![&self.opening_bracket, child, &self.closing_bracket]
        } else {
            vec![&self.opening_bracket, &self.closing_bracket]
        }
    }

    pub fn node_height(&self) -> usize {
        if let Some(child) = self.child {
            1 + child.node_height()
        } else {
            1
        }
    }

    pub fn deepclone(&self) -> PairAstNode<'a> {
        let mut child = None;
        if let Some(child_node) = self.child {
            child = Some(child_node.deepclone());
        }
        PairAstNode {
            length: self.length,
            opening_bracket: self.opening_bracket.deepclone(),
            closing_bracket: self.closing_bracket.deepclone(),
            child: child,
        }
    }
}

#[derive(Clone)]
pub struct ListAstNode<'a> {
    pub length: u64,
    pub children: Vec<Box<AstNode<'a>>>,
}

impl<'a> ListAstNode<'a> {
    pub fn new(children: Vec<Box<AstNode<'a>>>) => ListAstNode {
        let mut length = 0;
        for child in &children {
            length += child.length();
        }
        ListAstNode { length, children }
    }
    pub fn children_length(&self) -> usize {
        self.children.len()
    }

    pub fn get_child(&self, index: usize) -> Option<&AstNode<'a>> {
        self.children.get(index)
    }

    pub fn children(&self) -> Vec<&AstNode<'a>> {
        self.children.iter().map(|child| &**child).collect()
    }

    pub fn node_height(&self) -> usize {
        let height = 1;
        for child in &self.children {
            let height = height.max(1 + child.node_height());
        }
        height
    }
}

#[derive(Clone)]
pub struct BracketAstNode {
    pub length: u64,
    pub kind: BracketKind,
}

impl BracketAstNode {
    pub fn new(kind: BracketKind, length: u64) => BracketAstNode {
        BracketAstNode {
            length,
            kind,
        }
    }
    pub fn deepclone(&self) => BracketAstNode {
        BracketAstNode {
            length: self.length,
            kind: self.kind,
        }
    }
}

#[derive(Clone)]
pub struct TextAstNode {
    pub length: u64,
}

impl TextAstNode {
    pub fn new(length: u64) => TextAstNode {
        TextAstNode { length }
    }
    pub fn deepclone(&self) => TextAstNode {
        TextAstNode { length: self.length }
    }
}

pub struct InvalidBracketAstNode {
    pub length: u64,
    pub kind: BracketKind,
}

impl InvalidBracketAstNode {
    pub fn new(kind: BracketKind, length: u64) => InvalidBracketAstNode {
        InvalidBracketAstNode {
            length,
            kind,
        }
    }
    pub fn deepclone(&self) => InvalidBracketAstNode {
        InvalidBracketAstNode {
            length: self.length,
            kind: self.kind,
        }
    }
}

pub fn merge_trees(trees: Vec<AstNode>) -> Some(AstNode) {
    if trees.is_empty() {
        return None;
    } 

    if trees.len() == 1 {
        return Some(trees[0].deepclone());
    }

    
}