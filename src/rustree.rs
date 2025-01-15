use macroquad::prelude::*;

enum TreeNodeType {
	Struct,
	Leaf,
	Root,
	Raincatcher,
	SeedDropper,
}

struct TreeNode {
    node_type: TreeNodeType,
    size: i32,
    dist: f32,
    angle: i32,
    pos: (i32, i32),

    activated: bool,
}

pub struct Rustree {
    nodes: Vec<TreeNode>,
}

impl Rustree {
    pub fn new(x: i32, y: i32, size: i32) -> Rustree {
        let root_node: TreeNode = TreeNode {
            node_type: TreeNodeType::Struct,
            size,
            dist: 0.0,
            angle: 0,
            pos: (x, y),
            activated: false,
        };

        let mut node_list = Vec::new();
        node_list.push(root_node);

        Rustree {
            nodes: node_list,
        }
    }

    fn root(&self) -> &TreeNode {
        &self.nodes[0]
    }

    pub fn draw(&self) -> () {
        draw_circle(
            self.root().pos.0 as f32,
            self.root().pos.1 as f32,
            self.root().size as f32,
        BROWN
        );
    }
}
