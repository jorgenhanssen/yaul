#[derive(Debug)]
pub enum Instruction {
    Set(Source, Destination),
    Input(Destination),
    Output(Source),
    Add(Source, Source, Destination),
    Subtract(Source, Source, Destination),
    Multiply(Source, Source, Destination),
    Divide(Source, Source, Destination),
    Modulo(Source, Source, Destination),
    Jump(Label),
    JumpGreaterThan(Source, Source, Label),
    JumpEqual(Source, Source, Label),
    JumpLessThan(Source, Source, Label),
    Move(Source, Destination),
    Call(Label),
    Return,
    Time(Destination),
    Syscall(
        Destination,
        Source,
        Option<Source>,
        Option<Source>,
        Option<Source>,
        Option<Source>,
        Option<Source>,
        Option<Source>,
    ),
}

#[derive(Debug)]
pub enum Source {
    Address(usize),
    Reference(usize),
    Data(i64),
}

#[derive(Debug)]
pub enum Destination {
    Address(usize),
    Reference(usize),
}

#[derive(Debug)]
pub enum Label {
    Label(String),
    Instruction(usize),
}
