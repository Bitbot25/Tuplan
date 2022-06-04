pub trait CompileStage {
    type Input;
    type Result;

    fn run(input: Self::Input) -> Self::Result;
}
