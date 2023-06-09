use crate::parser::components::desc::Desc;

pub struct Class {
    pub desc: Option<Desc>,
    pub name: Option<String>,
}

impl Class {
    pub fn new() -> Self {
        Self {
            desc: None,
            name: None,
        }
    }
}
