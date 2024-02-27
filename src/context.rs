use garnish_lang::GarnishContext;
use garnish_lang::simple::{NoCustom, SimpleGarnishData};
use garnish_lang_utilities::DataInfoProvider;

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

impl DataInfoProvider<SimpleGarnishData> for BrowserContext {
    
}