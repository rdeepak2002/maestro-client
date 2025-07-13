use uuid::Uuid;

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Workflow {
    id: String,
}

impl Workflow {
    pub fn builder() -> WorkflowBuilder {
        WorkflowBuilder::default()
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct WorkflowBuilder {
    id: String,
}

impl Default for WorkflowBuilder {
    fn default() -> Self {
        WorkflowBuilder {
            id: Uuid::new_v4().to_string(),
        }
    }
}

impl WorkflowBuilder {
    pub fn id(mut self, id: String) -> WorkflowBuilder {
        self.id = id;
        self
    }

    pub fn build(self) -> Workflow {
        Workflow { id: self.id }
    }
}

#[test]
fn builder_test() {
    let expected = Workflow {
        id: String::from("x"),
    };
    let actual: Workflow = Workflow::builder().id("x".to_owned()).build();
    assert_eq!(expected, actual);
}
