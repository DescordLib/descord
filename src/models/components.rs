use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug, Default)]
pub struct Component {
    type_: u32,
    components: Vec<Component>,
}
