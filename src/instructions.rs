#[derive(Debug)]
pub enum Label {
    Label(String),
    Instruction(usize),
}

#[derive(Debug)]
pub enum Param {
    Address(usize),
    Reference(usize),
    Data(i64),
}

#[derive(Debug)]
pub enum Instruction {
    // value, destination
    Set(Param, Param),

    // destination
    Input(Param),

    // source
    Output(Param),

    // addend, addend, destination
    Add(Param, Param, Param),

    // minuend, subtrahend, destination
    Subtract(Param, Param, Param),

    // factor, factor, destination
    Multiply(Param, Param, Param),

    // dividend, divisor, destination
    Divide(Param, Param, Param),

    // dividend, divisor, register
    Modulo(Param, Param, Param),

    // target address
    Jump(Label),

    // A, B, destination (A > B)
    JumpGreaterThan(Param, Param, Label),

    // A, B, destination (A == B)
    JumpEqual(Param, Param, Label),

    // A, B, destination (A < B)
    JumpLessThan(Param, Param, Label),

    // source, destination
    Move(Param, Param),

    // none
    Terminate,

    // call label
    Call(Label),

    // none
    Return,

    // destination
    Time(Param),
}
