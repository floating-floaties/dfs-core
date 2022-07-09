use serde::{Serialize, Deserialize};
use std::{collections::{BTreeMap, HashSet}, vec};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Case {
    pub condition: String,
    pub reply: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Dialog {
    pub intent: String,
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Spec {
    pub intents: Vec<String>,
    pub context: BTreeMap<String, String>,
    pub dialogs: BTreeMap<String, Dialog>,
}

impl Case {
    pub fn new(condition: String, reply: String) -> Self {
        Case{
            condition,
            reply,
        }
    }

    pub fn default() -> Self {
        Case{
            condition: "true".to_owned(),
            reply: "This is a reply".to_owned(),
        }
    }
}

impl Dialog {
    pub fn new(intent: String, cases: Vec<Case>) -> Self {
        Dialog{
            intent,
            cases,
        }
    }

    pub fn default() -> Self {
        Dialog{
            intent: "billing".to_owned(),
            cases: vec![Case::default()],
        }
    }
}


impl Spec {
    pub fn new(intents: Vec<String>, dialogs: Vec<Dialog>, context: BTreeMap<String, String>) -> Self {
        let mut dialogs_map = BTreeMap::<String, Dialog>::new();
        for dialog in dialogs {
            if !intents.contains(&dialog.intent) {
                panic!("{} was not declared in the intents", dialog.intent);
            }
            if dialogs_map.contains_key(&dialog.intent) {
                panic!("{} has multiple dialogs", dialog.intent);
            }
            dialogs_map.insert(dialog.intent.to_owned(), dialog);
        }
        Spec{
            intents,
            dialogs: dialogs_map,
            context
        }
    }

    pub fn default() -> Self {
        let intents = vec!["billing".to_owned(), "commissions".to_owned(), "login issue".to_owned()];
        let dialogs = vec![
            Dialog::new("billing".to_owned(), vec![Case::default()]),
            Dialog::new("commissions".to_owned(), vec![Case::default()]),
            Dialog::new("login issue".to_owned(), vec![Case::default()]),
        ];
        let mut context = BTreeMap::<String, String>::new();
        context.insert("some_var".to_owned(), "42".to_owned());
        context.insert("something".to_owned(), "true".to_owned());
        Spec::new(intents, dialogs, context)
    }

    pub fn evaluate(&self) {
        
    }

    pub fn from_yaml(content: &String) -> Self {
        serde_yaml::from_str(&content).unwrap()
    }

    pub fn from_json(content: &String) -> Self {
        serde_json::from_str(&content).unwrap()
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }


    pub fn _write_to_yaml(&self, path: String) {
        std::fs::write(path, self.to_yaml()).expect("failed to write file");
    }

    pub fn _write_to_json(&self, path: String) {
        std::fs::write(path, self.to_json()).expect("failed to write file");
    }
}

fn main() {
    
    let user_spec = Spec::default();
    let s = serde_yaml::to_string(&user_spec).unwrap();
    println!("{}", s);

    std::fs::write("./examples/dialogs.yml", s).expect("failed to write file");
}
