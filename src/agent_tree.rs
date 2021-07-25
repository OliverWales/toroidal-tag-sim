use crate::agent;
use crate::vec;

// 2D-tree data structure intended to provide high speed neighbour search

type BoxedAgent = Box<dyn agent::Agent>;

pub trait AgentCollection<BoxedAgent> {
    fn new(agents: &Vec<BoxedAgent>) -> Self;
    fn get_in_rectilinear_range(&self, point: vec::Vec2, range: f64) -> Vec<BoxedAgent>;
    fn get_in_euclidean_range(&self, point: vec::Vec2, range: f64) -> Vec<BoxedAgent>;
}

pub struct AgentTree {
    root: Option<Box<AgentTreeNode>>,
}

pub struct AgentTreeNode {
    left: Option<Box<AgentTreeNode>>,
    right: Option<Box<AgentTreeNode>>,
    node: BoxedAgent,
}

impl Clone for AgentTreeNode {
    fn clone(&self) -> Self {
        AgentTreeNode {
            left: self.left.clone(),
            right: self.right.clone(),
            node: self.node.clone(),
        }
    }
}

impl AgentTree {
    fn range_search(
        &self,
        root: Option<Box<AgentTreeNode>>,
        min_x: f64,
        min_y: f64,
        max_x: f64,
        max_y: f64,
        x_axis: bool,
    ) -> Vec<BoxedAgent> {
        if root.is_none() {
            return Vec::new();
        }

        let root = root.unwrap();
        let mut result: Vec<BoxedAgent> = Vec::new();

        if x_axis {
            // if x <= max_x need to check right subtree
            if root.node.get_position().x <= max_x {
                result.append(
                    &mut self.range_search(root.right, min_x, min_y, max_x, max_y, !x_axis),
                );
            }
            // if x >= min_x need to check left subtree
            if root.node.get_position().x >= min_x {
                result
                    .append(&mut self.range_search(root.left, min_x, min_y, max_x, max_y, !x_axis));
            }
        } else {
            // if y <= max_y need to check right subtree
            if root.node.get_position().y <= max_y {
                result.append(
                    &mut self.range_search(root.right, min_x, min_y, max_x, max_y, !x_axis),
                );
            }
            // if y >= min_y need to check left subtree
            if root.node.get_position().y >= min_y {
                result
                    .append(&mut self.range_search(root.left, min_x, min_y, max_x, max_y, !x_axis));
            }
        }

        // if the agent is in range range add to the result
        if root.node.get_position().x >= min_x
            && root.node.get_position().y >= min_y
            && root.node.get_position().x <= max_x
            && root.node.get_position().y <= max_y
        {
            result.push(root.node);
        }

        return res;
    }
}

impl AgentCollection<BoxedAgent> for AgentTree {
    fn new(agents: &Vec<BoxedAgent>) -> Self {
        AgentTree {
            root: construct_tree(agents, true),
        }
    }

    fn get_in_rectilinear_range(&self, point: vec::Vec2, range: f64) -> std::vec::Vec<BoxedAgent> {
        let min_x = point.x - range;
        let min_y = point.y - range;
        let max_x = point.x + range;
        let max_y = point.y + range;

        if self.root.is_none() {
            return Vec::new();
        }

        return self.range_search(self.root.clone(), min_x, min_y, max_x, max_y, true);
    }

    fn get_in_euclidean_range(&self, point: vec::Vec2, range: f64) -> std::vec::Vec<BoxedAgent> {
        let candidates = self.get_in_rectilinear_range(point, range);

        return candidates
            .into_iter()
            .filter(|other| (point - other.get_position()).magnitude() <= range)
            .collect();
    }
}

fn construct_tree(agents: &Vec<BoxedAgent>, x_axis: bool) -> Option<Box<AgentTreeNode>> {
    if agents.len() == 0 {
        return None;
    }

    let mid = agents.len() / 2;

    let mut sorted = agents.clone();
    if x_axis {
        sorted.sort_by(|a, b| b.get_position().x.partial_cmp(&a.get_position().x).unwrap());
    } else {
        sorted.sort_by(|a, b| b.get_position().y.partial_cmp(&a.get_position().y).unwrap());
    }

    let left = construct_tree(&sorted[0..mid].to_vec(), !x_axis);
    let right = construct_tree(&sorted[mid + 1..agents.len()].to_vec(), !x_axis);

    return Some(Box::new(AgentTreeNode {
        left: left,
        right: right,
        node: agents[mid].clone_box(),
    }));
}
