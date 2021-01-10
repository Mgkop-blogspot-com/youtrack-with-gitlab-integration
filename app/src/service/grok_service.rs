use std::collections::HashMap;
use grok::Grok;

#[derive(Default)]
pub struct GrokService {
    grok: Grok
}

impl GrokService {
    pub fn new(patterns: HashMap<String, String>) -> GrokService {
        let mut grok = Grok::with_patterns();
        for (key, value) in patterns {
            grok.insert_definition(key, value)
        }
        GrokService { grok }
    }
}

#[cfg(test)]
mod test {
    use grok::Grok;

    #[test]
    fn check_mr_title() {
        let title_example = "2429 Gift Card belongs to deleted categories after disabling/enabling them";

        let mut grok = Grok::with_patterns();
        grok.insert_definition("TASK", r"[\d]+");
        grok.insert_definition("TITLE", r".+");

        let pattern = grok.compile("%{TASK} %{TITLE}", false)
            .expect("Error while compiling!");

        match pattern.match_against(title_example) {
            Some(m) =>
                println!(r#"Task: "{:?}"\n Title: "{:?}""#,
                         m.get("TASK"),
                         m.get("TITLE")),
            // Some(m) => println!("Found username {:?}", m.get("USERNAME")),
            None => println!("No matches found!"),
        }
    }
}