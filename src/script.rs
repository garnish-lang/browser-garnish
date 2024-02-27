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

    pub fn get_text(&self) -> String {
        self.text.clone()
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
}