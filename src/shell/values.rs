struct RuntimeVal {
    value_type: ValueType,
    value: Value

}
enum ValueType {
    Boolean,
    Number
}
enum Value {
    NumberVal(f32),
    BooleanVal(bool),
}