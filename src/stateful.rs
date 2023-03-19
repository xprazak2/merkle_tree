use crate::utils::create_hash;

#[derive(Clone)]
pub enum Item {
  Leaf(LeafInner),
  Node(NodeInner)
}

impl Item {
  pub fn is_leaf(&self) -> bool {
    if let Item::Leaf(_) = self {
        return true
    }
    return false
  }

  pub fn new_leaf(val: &str) -> Option<Box<Self>> {
    Some(Box::new(Self::Leaf(LeafInner { value: val.to_owned() })))
  }

  pub fn new_node(depth: u64, left: Option<Box<Self>>, right: Option<Box<Self>>) -> Option<Box<Self>> {
    Some(Box::new(Self::Node(NodeInner { depth, left, right, hashed_val: "".to_owned() })))
  }
}

#[derive(Clone)]
pub struct LeafInner {
  value: String
}

impl LeafInner {
  pub fn hashed_val(&self) -> String {
    create_hash(&self.value)
  }
}

#[derive(Clone)]
pub struct NodeInner {
  depth: u64,
  hashed_val: String,
  left: Option<Box<Item>>,
  right: Option<Box<Item>>
}

impl NodeInner {

  pub fn append(&mut self, val: &str) -> Option<Box<Item>> {
    let res = self.append_at_depth(val, self.depth);
    // check if returned node is self and create a new root if not
    // let new_parent = Item::new_node(self.depth + 1, Some(Box::new(Item::Node(self.clone()))), new_sibling);
    return res;
  }


  pub fn append_at_depth(&mut self, val: &str, depth: u64) -> Option<Box<Item>> {
    if self.left.is_none() {
      self.left = Some(Box::new(Item::Leaf(LeafInner { value: val.to_owned() })));
      self.depth = 1;
      return None;
    }

    if let Some(inner) = self.right.clone() {
      match *inner {
          Item::Leaf(inner_leaf) => {
            if (depth == self.depth) {
              let new_leaf = Item::new_leaf(val);
              let new_sibling = Item::new_node(self.depth, new_leaf, None);
              // let new_parent = Item::new_node(self.depth + 1, Some(Box::new(Item::Node(self.clone()))), new_sibling);
              return new_sibling;
            } else {
              let new_leaf = Item::new_leaf(val);
              let new_sibling = Item::new_node(self.depth, new_leaf, None);
              return new_sibling;
            }
          }
          Item::Node(mut inner_node) => {
            let node = inner_node.append_at_depth(val, depth);
            return Item::new_node(self.depth, node, None);
          }
      }
    } else {
      self.right = Some(Box::new(Item::Leaf(LeafInner { value: val.to_owned() })));
      return None;
    }

    None
  }
}