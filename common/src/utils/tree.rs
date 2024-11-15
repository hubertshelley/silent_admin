use serde::Serialize;

pub trait TreeNode: Serialize + Clone {
    fn id(&self) -> String;
    fn parent_id(&self) -> Option<String>;
}

#[derive(Serialize, Debug, Clone)]
pub struct Tree<T>
where
    T: TreeNode,
{
    #[serde(flatten)]
    data: T,
    children: Vec<Tree<T>>,
}

impl<T> Tree<T>
where
    T: TreeNode,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            children: vec![],
        }
    }
    pub fn build(data: Vec<T>, root_id: Option<String>) -> Vec<Tree<T>> {
        Self::build_tree(data.into_iter().map(|d| Tree::new(d)).collect(), root_id)
    }
    fn build_tree(data: Vec<Tree<T>>, parent_id: Option<String>) -> Vec<Tree<T>> {
        let mut tree = Vec::new();
        for mut item in data.clone() {
            if item.data.parent_id() == parent_id {
                item.children = Self::build_tree(data.clone(), Some(item.data.id()));
                tree.push(item.clone());
            }
        }
        tree
    }
}
