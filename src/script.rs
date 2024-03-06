use crate::context::BrowserContext;
use garnish_lang::compiler::build::build_with_data;
use garnish_lang::compiler::lex::{lex};
use garnish_lang::compiler::parse::parse;
use garnish_lang::simple::{SimpleGarnishData, SimpleGarnishRuntime, SimpleRuntimeState};
use garnish_lang::{GarnishData, GarnishRuntime};
use garnish_lang_utilities::simple_expression_data_format;
use wasm_bindgen::prelude::wasm_bindgen;
use garnish_lang_utilities::data::copy_data_at_to_data;
use crate::compile::compile_source_into_data;

#[wasm_bindgen]
pub struct SourceDetails {
    name: String,
    text: String
}

#[wasm_bindgen]
impl SourceDetails {
    pub fn new(name: String, text: String) -> Self {
        SourceDetails { name, text }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text
    }
}

impl SourceDetails {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

#[wasm_bindgen]
pub struct GarnishScript {
    source: SourceDetails,
    input: Option<String>,
    include: Vec<SourceDetails>,
    data: SimpleGarnishData,
    error: Option<String>,
    executions: Vec<SimpleGarnishData>,
    context: BrowserContext,
    execution_limit: usize,
}

#[wasm_bindgen]
impl GarnishScript {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, text: String) -> Self {
        GarnishScript {
            source: SourceDetails::new(name, text),
            input: None,
            include: vec![],
            data: SimpleGarnishData::new(),
            error: None,
            executions: vec![],
            context: BrowserContext::new(),
            execution_limit: 10000
        }
    }

    pub fn get_name(&self) -> String {
        self.source.get_name()
    }

    pub fn set_name(&mut self, name: String) {
        self.source.set_name(name);
    }

    pub fn get_text(&self) -> String {
        self.source.get_text()
    }

    pub fn set_text(&mut self, text: String) {
        self.source.set_text(text);
    }

    pub fn get_input(&self) -> Option<String> {
        self.input.clone()
    }

    pub fn set_input(&mut self, input: String) {
        self.input = Some(input);
    }

    pub fn get_error(&self) -> Option<String> {
        self.error.clone()
    }

    pub fn include(&mut self, name: String, text: String) {
        self.include.push(SourceDetails::new(name, text))
    }

    pub fn get_execution_result(&self, execution_index: usize) -> Option<String> {
        self.executions.get(execution_index).and_then(|execution| {
            execution.get_current_value().and_then(|v| {
                Some(simple_expression_data_format(
                    v,
                    execution,
                    &self.context,
                    0,
                ))
            })
        })
    }

    pub fn get_execution_count(&self) -> u32 {
        self.executions.len() as u32
    }

    pub fn clear_executions(&mut self) {
        self.executions = vec![];
    }

    pub fn compile(&mut self) {
        self.data = SimpleGarnishData::new_custom();

        match compile_source_into_data(&self.source, &mut self.data, &mut self.context) {
            Err(e) => {
                self.error = Some(format!("Error compiling {}: {}", self.source.name(), e));
                return;
            },
            Ok(()) => ()
        }

        for source in &self.include {
            match compile_source_into_data(source, &mut self.data, &mut self.context) {
                Err(e) => {
                    self.error = Some(format!("Error compiling {}: {}", self.source.name(), e));
                    return;
                },
                Ok(()) => ()
            }
        }
    }

    pub fn execute(&mut self) {
        let mut execution_data = self.data.clone();
        let input_addr = match self.make_input() {
            Err(e) if e == String::from("No Input") => 0,
            Err(e) => {
                self.error = Some(e);
                return;
            }
            Ok(data) => match data.get_current_value() {
                None => {
                    self.error = Some(String::from("No current value made for input."));
                    return;
                }
                Some(i) => match copy_data_at_to_data(i, &data, &mut execution_data) {
                    Err(e) => {
                        self.error = Some(e.to_string());
                        return;
                    }
                    Ok(i) => i,
                }
            }
        };

        match execution_data.push_value_stack(input_addr) {
            Err(e) => {
                self.error = Some(e.to_string());
                return;
            }
            Ok(()) => (),
        }

        let mut runtime = SimpleGarnishRuntime::new(execution_data);

        let limit = self.execution_limit;
        let mut count = 0;

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

            count += 1;
            if count >= limit {
                self.error = Some("Instruction execution limit reached. Possibly an infinite loop.".to_string());
                break;
            }
        }

        self.executions.push(runtime.get_data().clone());
    }

    fn make_input(&mut self) -> Result<SimpleGarnishData, String> {
        match self.get_input() {
            None => Err(String::from("No Input")),
            Some(input) => {
                let data = self.compile_script(&input)?;
                let data = self.execute_data(data);
                Ok(data)
            }
        }
    }

    fn compile_script(&mut self, text: &str) -> Result<SimpleGarnishData, String> {
        let tokens = match lex(text) {
            Ok(tokens) => tokens,
            Err(e) => {
                return Err(e.get_message().clone());
            }
        };

        let parse_result = match parse(&tokens) {
            Err(e) => {
                return Err(e.get_message().clone());
            }
            Ok(result) => result,
        };

        let mut data = SimpleGarnishData::new_custom();

        match build_with_data(
            parse_result.get_root(),
            parse_result.get_nodes().clone(),
            &mut data,
        ) {
            Err(e) => {
                return Err(e.get_message().clone());
            }
            Ok(_) => {}
        }

        return Ok(data);
    }

    fn execute_data(&mut self, mut data: SimpleGarnishData) -> SimpleGarnishData {
        match data.push_value_stack(0) {
            Err(e) => {
                self.error = Some(e.to_string());
                return data;
            }
            Ok(()) => {}
        }

        let mut runtime = SimpleGarnishRuntime::new(data);

        let limit = self.execution_limit;
        let mut count = 0;

        loop {
            match runtime.execute_current_instruction(Some(&mut self.context)) {
                Err(e) => {
                    self.error = Some(e.get_message().clone());
                    break;
                }
                Ok(info) => match info.get_state() {
                    SimpleRuntimeState::Running => (),
                    SimpleRuntimeState::End => break,
                },
            }

            count += 1;
            if count >= limit {
                self.error = Some("Instruction execution limit reached. Possibly an infinite loop.".to_string());
                break;
            }
        }

        return runtime.get_data_owned();
    }
}

// for methods that won't be exposed to JS
// allowing dead to suppress warning for wasm build
#[allow(dead_code)]
impl GarnishScript {

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
    fn set_input() {
        let mut script = GarnishScript::new("test_one".to_string(), "5 + 5".to_string());
        script.set_input("10 20".to_string());

        assert_eq!(script.get_input(), Some("10 20".to_string()));
    }

    #[test]
    fn compile() {
        let mut script = GarnishScript::new("test_one".to_string(), "5 + 5".to_string());
        script.compile();

        assert_eq!(script.get_data().get_data().len(), 4);
    }

    #[test]
    fn compile_with_error() {
        let mut script = GarnishScript::new("test_one".to_string(), "(5 + 5".to_string());
        script.compile();

        assert_eq!(
            script.get_error(),
            Some("Error compiling test_one: Syntax Error: Unclosed grouping".to_string())
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

    #[test]
    fn get_execution_count() {
        let mut script = GarnishScript::new("test_one".to_string(), "5 + 5".to_string());
        script.compile();
        assert_eq!(script.get_execution_count(), 0);

        script.execute();
        assert_eq!(script.get_execution_count(), 1);

        script.execute();
        assert_eq!(script.get_execution_count(), 2);
    }

    #[test]
    fn clear_executions() {
        let mut script = GarnishScript::new("test_one".to_string(), "5 + 5".to_string());
        script.compile();
        script.execute();
        script.execute();
        script.execute();
        assert_eq!(script.get_execution_count(), 3);

        script.clear_executions();
        assert_eq!(script.get_execution_count(), 0);
    }

    #[test]
    fn execute_with_input() {
        let mut script = GarnishScript::new("test_one".to_string(), "$ + 5".to_string());
        script.set_input("10".to_string());
        script.compile();
        script.execute();

        let v = script
            .get_execution(0)
            .unwrap()
            .get_current_value()
            .unwrap();
        assert_eq!(
            script.get_execution(0).unwrap().get_data().get(v).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(15))
        )
    }

    #[test]
    fn execute_with_includes() {
        let mut script = GarnishScript::new("test_one".to_string(), "add_5 ~ 5".to_string());
        script.include("add_5".to_string(), "$ + 5".to_string());
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
    fn execute_with_def() {
        let mut script = GarnishScript::new("test_one".to_string(), "@Def add_5 { $ + 5 }\n\nadd_5 ~ 5".to_string());
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
    fn execute_with_def_multiple_roots() {
        let mut script = GarnishScript::new("test_one".to_string(), "5 * 10\n\n@Def add_5 { $ + 5 }\n\nadd_5 ~ $".to_string());
        script.compile();
        script.execute();

        let v = script
            .get_execution(0)
            .unwrap()
            .get_current_value()
            .unwrap();
        assert_eq!(
            script.get_execution(0).unwrap().get_data().get(v).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(55))
        )
    }

    #[test]
    fn resolve_context_value() {
        let mut script = GarnishScript::new("test_one".to_string(), "Math::IntegerMax".to_string());
        script.compile();
        script.execute();

        let v = script
            .get_execution(0)
            .unwrap()
            .get_current_value()
            .unwrap();
        assert_eq!(
            script.get_execution(0).unwrap().get_data().get(v).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(i32::MAX))
        )
    }

    #[test]
    fn execution_limit() {
        let mut script = GarnishScript::new("test_one".to_string(), "$? ^~ $ + 5".to_string());
        script.compile();
        script.execute();

        assert_eq!(script.error, Some("Instruction execution limit reached. Possibly an infinite loop.".to_string()))
    }
}
