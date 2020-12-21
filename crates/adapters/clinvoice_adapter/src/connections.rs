use std::collections::HashMap;

pub type Connections<'name, 'url> = HashMap<&'name str, &'url str>;
