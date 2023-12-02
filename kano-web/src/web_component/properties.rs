use std::collections::HashMap;

use kano::DeserializeAttribute;

pub type ComponentProperties = HashMap<String, String>;

pub fn read_props<A: DeserializeAttribute>(props: &ComponentProperties) -> Vec<Option<A>> {
    let mut output = vec![];

    for (name, value) in props {
        if let Some(property) = A::deserialize(name, value.clone()) {
            output.push(Some(property));
        }
    }

    output
}
