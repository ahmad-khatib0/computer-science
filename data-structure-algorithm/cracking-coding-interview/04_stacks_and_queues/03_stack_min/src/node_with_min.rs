pub struct NodeWithMin {
    pub value: i32,
    pub min: i32,
}

impl NodeWithMin {
    pub fn new(value: i32, min: i32) -> Self {
        NodeWithMin { value, min }
    }
}
