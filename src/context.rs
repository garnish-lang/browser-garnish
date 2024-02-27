use garnish_lang::GarnishContext;
use garnish_lang::simple::{NoCustom, SimpleGarnishData};

pub struct BrowserContext {

}

impl BrowserContext {
    pub fn new() -> Self {
        BrowserContext {

        }
    }
}

impl GarnishContext<SimpleGarnishData<NoCustom>> for BrowserContext {

}