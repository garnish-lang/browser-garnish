use garnish_lang::compiler::build::build_with_data;
use garnish_lang::compiler::lex::{lex, Lexer, LexerToken};
use garnish_lang::compiler::parse::parse;
use garnish_lang::simple::SimpleGarnishData;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
struct GarnishScript {
    name: String,
    text: String,
    source_tokens: Vec<LexerToken>,
    data: SimpleGarnishData,
    error: Option<String>,
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

    pub fn compile(&mut self) {
        self.source_tokens = match lex(&self.text) {
            Ok(tokens) => tokens,
            Err(e) => {
                self.error = Some(e.to_string());
                return;
            }
        };

        let parse_result = match parse(&self.source_tokens) {
            Err(e) => {
                self.error = Some(e.to_string());
                return;
            }
            Ok(result) => result,
        };

        self.data = SimpleGarnishData::new();

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
}

// for methods that won't be exposed to JS
impl GarnishScript {
    pub fn get_source_tokens(&self) -> &Vec<LexerToken> {
        &self.source_tokens
    }

    pub fn get_data(&self) -> &SimpleGarnishData {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::script::GarnishScript;

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
}
