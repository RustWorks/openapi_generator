#![allow(clippy::clone_on_copy)]

pub mod components {
{{~#with components}}
    pub mod schemas {
        use super::super::components;
        use serde::{Deserialize, Serialize};

        {{~#each schemas}}
            {{>schema name=@key this}}
        {{~/each}}
    }
{{~/with}}
}