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

struct Rustree {
    root: (i32, i32),
}
