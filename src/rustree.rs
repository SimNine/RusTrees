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

    // owner: Option<&Rustree>,
    // parent: Option<&TreeNode>,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(
        node_type: TreeNodeType,
        size: i32,
        dist: f32,
        angle: i32,
        pos: (i32, i32),
        activated: bool,
    ) -> TreeNode {
        TreeNode {
            node_type,
            size,
            dist,
            angle,
            pos,
            activated,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }
}



pub struct Rustree {
    root: TreeNode,
}

impl Rustree {
    pub fn new(
        x: i32,
        y: i32,
    ) -> Rustree {
        let root_node = TreeNode::new(
            TreeNodeType::Root,
            30,
            0.0,
            0,
            (x, y),
            false,
        );

        Rustree {
            root: root_node,
        }
    }

    pub fn new_from_data(
        n: TreeNode,
    ) -> Rustree {
        Rustree {
            root: n,
        }
    }

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
        for node in &self.nodes {
            let color: macroquad::prelude::Color;
            match node.node_type {
                TreeNodeType::Struct => {
                    color = BROWN;
                }
                _ => {
                    color = BLACK;
                }
            }

            draw_circle(
                node.pos.0 as f32,
                node.pos.1 as f32,
                node.size as f32,
                color,
            );
        }
        // draw_circle(
        //     self.root().pos.0 as f32,
        //     self.root().pos.1 as f32,
        //     self.root().size as f32,
        // BROWN
        // );
    }
}
