#[derive(Debug)]
pub struct CookieTab {
    pub name: String,
}

impl CookieTab {
    pub fn new() -> Self {
        const COOKIE_TAB_NAME: &str = "Cookies";
        CookieTab {
            name: COOKIE_TAB_NAME.to_string(),
        }
    }
}
