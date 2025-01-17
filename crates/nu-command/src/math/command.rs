use nu_engine::get_full_help;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, Signature, Value,
};

#[derive(Clone)]
pub struct MathCommand;

impl Command for MathCommand {
    fn name(&self) -> &str {
        "math"
    }

    fn signature(&self) -> Signature {
        Signature::build("math")
    }

    fn usage(&self) -> &str {
        "Use mathematical functions as aggregate functions on a list of numbers or tables."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        Ok(Value::String {
            val: get_full_help(
                &MathCommand.signature(),
                &MathCommand.examples(),
                engine_state,
            ),
            span: call.head,
        }
        .into_pipeline_data())
    }
}
