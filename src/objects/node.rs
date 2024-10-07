use std::borrow::Cow;
use std::io;
use ptree2::{Style, TreeItem};
use crate::objects::commit::Commit;

#[derive(Clone)]
pub struct Node {
    commit: Commit,
    children: Vec<Node>
}

impl Node {
    pub fn new(commit: Commit, children: Vec<Node>) -> Node{
        Node {
            commit,
            children
        }
    }

    pub fn add_child_to_tree(&mut self, node: &Node){

        if self.commit.get_hash() == node.commit.get_parent() {
            self.children.push(node.clone());
        }
        else {
            for n in self.children.iter_mut() {
                n.add_child_to_tree(node);
            }
        }
    }
    
    pub fn display(&self) {
        self.commit.display();
        for n in self.children.iter() {
            n.display()
        }
    }

}

impl TreeItem for Node {
    type Child = Self;
    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        write!(f, "id: {}; desc: {}", style.paint(self.commit.get_hash()), style.paint(self.commit.get_description()))
    }

    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(&self.children)
    }
}