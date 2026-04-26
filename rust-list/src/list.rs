pub enum List<TypeName> {
    Node(TypeName, Box<List<TypeName>>),
    End,
}

impl<TypeName> List<TypeName>
where
    TypeName: std::fmt::Display,
{
    pub fn new() -> List<TypeName> {
        List::End
    }

    // 在链表头部插入一个节点
    pub fn prepend(self, element: TypeName) -> List<TypeName> {
        List::Node(element, Box::new(self))
    }

    // 获取链表的长度
    pub fn len(&self) -> usize {
        match self {
            List::Node(_, rest) => 1 + rest.len(),
            List::End => 0,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            List::Node(value, rest) => format!("{} -> {}", value, rest.to_string()),
            List::End => "End".to_string(),
        }
    }
}
