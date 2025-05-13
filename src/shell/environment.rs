use super::values::Value;
use std::collections::HashMap;

struct LocalEnv {
    frames: Vec<Frame>
}

struct GobalEnv {
    variables: HashMap<String, Value>,
    //functions: HashMap<String, Function>,
}

struct ShellEnv {
    globals: GobalEnv,
    locals: LocalEnv
}

struct Frame {
    name: Option<String>,
    vars: HashMap<String, Value>
}