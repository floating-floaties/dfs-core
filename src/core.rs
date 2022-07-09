#![allow(dead_code)]

pub mod spec {
    use chrono::{Datelike, Timelike};
    use std::{
        collections::{BTreeMap, HashMap},
        vec,
    };

    use eval::{to_value, Expr, Value};
    use serde::{Deserialize, Serialize};
    use regex::Regex;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Case {
        pub condition: String,
        pub reply: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Dialog {
        pub intent: String,
        pub cases: Vec<Case>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Spec {
        pub intents: Vec<String>,
        pub context: HashMap<String, String>,
        pub dialogs: BTreeMap<String, Dialog>,
    }

    impl Case {
        pub fn new(condition: String, reply: String) -> Self {
            Case { condition, reply }
        }

        pub fn default() -> Self {
            Case {
                condition: "true".to_owned(),
                reply: "This is a reply".to_owned(),
            }
        }
    }

    impl Dialog {
        pub fn new(intent: String, cases: Vec<Case>) -> Self {
            Dialog { intent, cases }
        }
    }

    impl Spec {
        pub fn new(
            intents: Vec<String>,
            dialogs: Vec<Dialog>,
            context: HashMap<String, String>,
        ) -> Self {
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
            Spec {
                intents,
                dialogs: dialogs_map,
                context,
            }
        }

        pub fn default() -> Self {
            let intents = vec![
                "billing".to_owned(),
                "commissions".to_owned(),
                "login issue".to_owned(),
            ];
            let dialogs = vec![
                Dialog::new("billing".to_owned(), vec![Case::default()]),
                Dialog::new("commissions".to_owned(), vec![Case::default()]),
                Dialog::new("login issue".to_owned(), vec![Case::default()]),
            ];
            let mut context = HashMap::<String, String>::new();
            context.insert("some_var".to_owned(), "42".to_owned());
            context.insert("something".to_owned(), "true".to_owned());
            Spec::new(intents, dialogs, context)
        }

        pub fn expr(&self, expression: String) -> Expr {
            Expr::new(expression)
                .value("ctx", &self.context)
                .function("int", |value| {
                    let v = value.get(0).unwrap();
                    let num: i64 = match v {
                        Value::Number(x) => x.as_i64().unwrap(),
                        Value::Bool(x) => if *x {1} else {0},
                        Value::String(x) => {
                            match x.parse::<i64>() {
                                Ok(x) => x,
                                _ => std::i64::MIN,
                            }
                        },
                        _ => std::i64::MIN,
                    };
                    Ok(to_value(num))
                })
                .function("float", |value| {
                    let v = value.get(0).unwrap();
                    let num: f64 = match v {
                        Value::Number(x) => x.as_f64().unwrap(),
                        Value::Bool(x) => if *x {1.0} else {0.0},
                        Value::String(x) => {
                            match x.parse::<f64>() {
                                Ok(x) => x,
                                _ => std::f64::NAN,
                            }
                        },
                        _ => std::f64::NAN,
                    };
                    Ok(to_value(num))
                })
                .function("bool", |value| {
                    let v = value.get(0).unwrap();
                    let result: bool = match v {
                        Value::Number(x) => x.as_f64().unwrap() != 0.0,
                        Value::Bool(x) => *x,
                        Value::String(x) => !x.is_empty(),
                        Value::Array(x) => !x.is_empty(),
                        Value::Object(x) => !x.is_empty(),
                        _ => false,
                    };

                    Ok(to_value(result))
                })
                .function("str", |value| {
                    let v = value.get(0).unwrap();
                    let result: String = match v {
                        Value::Number(x) => x.as_f64().unwrap().to_string(),
                        Value::Bool(x) => x.to_string(),
                        Value::String(x) => x.to_string(),
                        Value::Array(x) => serde_json::to_string(x).unwrap(),
                        Value::Object(x) => serde_json::to_string(x).unwrap(),
                        _ => String::from("null"),
                    };
                    Ok(to_value(result))
                })
                .function("day", |_| {
                    let current_time = chrono::offset::Local::now();
                    Ok(to_value(current_time.date().day()))
                })
                .function("month", |_| {
                    let current_time = chrono::offset::Local::now();
                    Ok(to_value(current_time.date().month()))
                })
                .function("year", |_| {
                    let current_time = chrono::offset::Local::now();
                    Ok(to_value(current_time.date().year()))
                })
                .function("weekday", |_| {
                    let current_time = chrono::offset::Local::now();
                    Ok(to_value(current_time.date().weekday().number_from_monday()))
                })
                .function("is_weekday", |_| {
                    let current_time = chrono::offset::Local::now();
                    let weekday = current_time.date().weekday().number_from_monday();
                    Ok(to_value(weekday < 6))
                })
                .function("time", |extract| {
                    let v = extract.get(0).unwrap().as_str().unwrap().to_lowercase();
                    let current_time = chrono::offset::Local::now().time();
                    let result = match v.as_str() {
                        "h" | "hour" | "hours" => current_time.hour(),
                        "m" | "minute" | "minutes" => current_time.minute(),
                        "s" | "second" | "seconds" => current_time.second(),
                        _ => current_time.hour(),
                    };
                    Ok(to_value(result))
                })
                .function("is_match", |value| {
                    let v = value.get(0).unwrap();
                    let pattern = value.get(1).unwrap().to_string();

                    let value: String = match v {
                        Value::Number(x) => x.as_f64().unwrap().to_string(),
                        Value::Bool(x) => x.to_string(),
                        Value::String(x) => x.to_string(),
                        Value::Array(x) => serde_json::to_string(x).unwrap(),
                        Value::Object(x) => serde_json::to_string(x).unwrap(),
                        _ => String::from("null"),
                    };

                    let prog = Regex::new(&pattern).unwrap();
                    let is_match = prog.is_match(&value);

                    Ok(to_value(is_match))
                })
        }

        pub fn eval<S: AsRef<str>>(&self, expression: S) -> Value {
            let str_like = expression.as_ref().to_owned();
            let result = self.expr(str_like).exec();

            if result.is_err() {
                panic!(
                    "Failed to parse expression: \"{}\" {:?}",
                    expression.as_ref().to_owned(),
                    result
                )
            }

            return result.unwrap();
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

        pub fn write_to_yaml(&self, path: String) {
            std::fs::write(path, self.to_yaml()).expect("failed to write file");
        }

        pub fn write_to_json(&self, path: String) {
            std::fs::write(path, self.to_json()).expect("failed to write file");
        }
    }
}
