use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Example, PipelineData, ShellError, Signature, Span, Type, Value,
};

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "split chars"
    }

    fn signature(&self) -> Signature {
        Signature::build("split chars")
    }

    fn usage(&self) -> &str {
        "splits a string's characters into separate rows"
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Split the string's characters into separate rows",
            example: "'hello' | split chars",
            result: Some(Value::List {
                vals: vec![
                    Value::test_string("h"),
                    Value::test_string("e"),
                    Value::test_string("l"),
                    Value::test_string("l"),
                    Value::test_string("o"),
                ],
                span: Span::unknown(),
            }),
        }]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        split_chars(engine_state, call, input)
    }
}

fn split_chars(
    engine_state: &EngineState,
    call: &Call,
    input: PipelineData,
) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
    let span = call.head;

    input.flat_map(
        move |x| split_chars_helper(&x, span),
        engine_state.ctrlc.clone(),
    )
}

fn split_chars_helper(v: &Value, name: Span) -> Vec<Value> {
    match v.span() {
        Ok(v_span) => {
            if let Ok(s) = v.as_string() {
                s.chars()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(move |x| Value::string(x, v_span))
                    .collect()
            } else {
                vec![Value::Error {
                    error: ShellError::PipelineMismatch {
                        expected: Type::String,
                        expected_span: name,
                        origin: v_span,
                    },
                }]
            }
        }
        Err(error) => vec![Value::Error { error }],
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand {})
    }
}
