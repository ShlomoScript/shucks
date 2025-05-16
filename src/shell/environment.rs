use super::values::Value;
use std::collections::HashMap;

struct LocalEnv {
    frames: Vec<Frame>
}

struct GlobalEnv {
    variables: HashMap<String, Value>,
    //functions: HashMap<String, Function>,
}

pub struct ShellEnv {
    globals: GlobalEnv,
    locals: LocalEnv
}

struct Frame {
    name: Option<String>,
    vars: HashMap<String, Value>
}
impl ShellEnv {
    pub fn new() -> Self {
        ShellEnv {
            globals: GlobalEnv::new(),
            locals: LocalEnv::new()
        }
    }
}
impl GlobalEnv {
    pub fn new() -> Self {
        GlobalEnv {
            variables: HashMap::new()
        }
    }
}
impl LocalEnv {
    pub fn new() -> Self {
        LocalEnv {
            frames: Vec::new()
        }
    }
}