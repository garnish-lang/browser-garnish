use wasm_bindgen::prelude::wasm_bindgen;
use garnish_lang::simple::SimpleGarnishData;

#[wasm_bindgen]
struct GarnishScript {
    text: String,
    data: SimpleGarnishData,
}

#[wasm_bindgen]
impl GarnishScript {
    #[wasm_bindgen(constructor)]
    pub fn new(text: String) -> Self {
        GarnishScript {
            text,
            data: SimpleGarnishData::new_custom(),
        }
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::script::GarnishScript;

    #[test]
    fn new_and_get_text() {
        let script = GarnishScript::new("5 + 5".to_string());
        assert_eq!(script.get_text(), "5 + 5".to_string())
    }
}