use crate::context::BrowserContext;
use garnish_lang::compiler::build::build_with_data;
use garnish_lang::compiler::lex::{lex, LexerToken};
use garnish_lang::compiler::parse::parse;
use garnish_lang::simple::{SimpleGarnishData, SimpleGarnishRuntime, SimpleRuntimeState};
use garnish_lang::{GarnishData, GarnishRuntime};
use wasm_bindgen::prelude::wasm_bindgen;
use garnish_lang_utilities::simple_expression_data_format;

#[wasm_bindgen]
struct GarnishScript {
    name: String,
    text: String,
    source_tokens: Vec<LexerToken>,
    data: SimpleGarnishData,
    error: Option<String>,
    executions: Vec<SimpleGarnishData>,
    context: BrowserContext,
}

#[wasm_bindgen]
impl GarnishScript {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, text: String) -> Self {
        GarnishScript {
            name,
            text,
            source_tokens: vec![],
            data: SimpleGarnishData::new(),
            error: None,
            executions: vec![],
            context: BrowserContext::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn get_error(&self) -> Option<String> {
        self.error.clone()
    }

    pub fn get_execution_result(&self, execution_index: usize) -> Option<String> {
        self.executions.get(execution_index).and_then(|execution| {
            execution
                .get_current_value()
                .and_then(|v| Some(simple_expression_data_format(v, execution, &self.context, 0)))
        })
    }

    pub fn compile(&mut self) {
        self.source_tokens = match lex(&self.text) {
            Ok(tokens) => tokens,
            Err(e) => {
                self.error = Some(e.get_message().clone());
                return;
            }
        };

        let parse_result = match parse(&self.source_tokens) {
            Err(e) => {
                self.error = Some(e.get_message().clone());
                return;
            }
            Ok(result) => result,
        };

        self.data = SimpleGarnishData::new_custom();

        match build_with_data(
            parse_result.get_root(),
            parse_result.get_nodes().clone(),
            &mut self.data,
        ) {
            Err(e) => {
                self.error = Some(e.to_string());
                return;
            }
            Ok(_) => {}
        }
    }

    pub fn execute(&mut self) {
        let mut execution_data = self.data.clone();
        match execution_data.push_value_stack(0) {
            Err(e) => {
                self.error = Some(e.to_string());
                return;
            }
            Ok(()) => {}
        }

        let mut runtime = SimpleGarnishRuntime::new(execution_data);

        loop {
            match runtime.execute_current_instruction(Some(&mut self.context)) {
                Err(e) => {
                    self.error = Some(e.get_message().clone());
                    return;
                }
                Ok(data) => match data.get_state() {
                    SimpleRuntimeState::Running => (),
                    SimpleRuntimeState::End => break,
                },
            }
        }

        self.executions.push(runtime.get_data().clone());
    }
}

// for methods that won't be exposed to JS
// allowing dead to suppress warning for wasm build
#[allow(dead_code)]
impl GarnishScript {
    pub fn get_source_tokens(&self) -> &Vec<LexerToken> {
        &self.source_tokens
    }

    pub fn get_data(&self) -> &SimpleGarnishData {
        &self.data
    }

    pub fn get_execution(&self, index: usize) -> Option<&SimpleGarnishData> {
        self.executions.get(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::script::GarnishScript;
    use garnish_lang::simple::{SimpleData, SimpleNumber};
    use garnish_lang::GarnishData;

    #[test]
    fn new_and_get_text_name() {
        let script = GarnishScript::new("test_one".to_string(), "5 + 5".to_string());
        assert_eq!(script.get_name(), "test_one".to_string());
        assert_eq!(script.get_text(), "5 + 5".to_string())
    }

    #[test]
    fn set_name() {
        let mut script = GarnishScript::new("test_one".to_string(), "5 + 5".to_string());
        script.set_name("test_two".to_string());

        assert_eq!(script.get_name(), "test_two".to_string());
    }

    #[test]
    fn set_text() {
        let mut script = GarnishScript::new("test_one".to_string(), "5 + 5".to_string());
        script.set_text("10 + 10".to_string());

        assert_eq!(script.get_text(), "10 + 10".to_string());
    }

    #[test]
    fn compile() {
        let mut script = GarnishScript::new("test_one".to_string(), "5 + 5".to_string());
        script.compile();

        assert_eq!(script.get_source_tokens().len(), 5);
        assert_eq!(script.get_data().get_data().len(), 4);
    }

    #[test]
    fn compile_with_error() {
        let mut script = GarnishScript::new("test_one".to_string(), "(5 + 5".to_string());
        script.compile();

        assert_eq!(
            script.get_error(),
            Some("Syntax Error: Unclosed grouping".to_string())
        );
    }

    #[test]
    fn execute() {
        let mut script = GarnishScript::new("test_one".to_string(), "5 + 5".to_string());
        script.compile();
        script.execute();

        let v = script
            .get_execution(0)
            .unwrap()
            .get_current_value()
            .unwrap();
        assert_eq!(
            script.get_execution(0).unwrap().get_data().get(v).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(10))
        )
    }

    #[test]
    fn formatted_execution_value() {
        let mut script = GarnishScript::new("test_one".to_string(), "5 + 5".to_string());
        script.compile();
        script.execute();

        assert_eq!(script.get_execution_result(0), Some("10".to_string()));
        assert_eq!(script.get_execution_result(1), None);
    }
}
