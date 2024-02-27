use wasm_bindgen::prelude::wasm_bindgen;
use garnish_lang::simple::SimpleGarnishData;

#[wasm_bindgen]
struct GarnishScript {
    name: String,
    text: String,
    data: SimpleGarnishData,
}

#[wasm_bindgen]
impl GarnishScript {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, text: String) -> Self {
        GarnishScript {
            name,
            text,
            data: SimpleGarnishData::new_custom(),
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
}