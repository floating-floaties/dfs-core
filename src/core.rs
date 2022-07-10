#![allow(dead_code)]

pub mod spec {
    use std::{
        collections::{BTreeMap, HashMap},
        num::IntErrorKind,
        vec,
    };

    use chrono::{Datelike, Timelike};
    use eval::{to_value, Expr, Value};
    use regex::Regex;
    use serde::{Deserialize, Serialize};

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
                    if value.is_empty() {
                        return Ok(to_value(0));
                    }
                    let v = value.get(0).unwrap();
                    let num: i64 = match v {
                        Value::Number(x) => {
                            if x.is_f64() {
                                x.as_f64().unwrap() as i64
                            } else {
                                x.as_i64().unwrap()
                            }
                        }
                        Value::Bool(x) => {
                            if *x {
                                1
                            } else {
                                0
                            }
                        }
                        Value::String(x) => _atoi(x.to_string()),
                        _ => 0,
                    };
                    Ok(to_value(num))
                })
                .function("float", |value| {
                    if value.is_empty() {
                        return Ok(to_value(std::f64::NAN));
                    }
                    let v = value.get(0).unwrap();
                    let num: f64 = match v {
                        Value::Number(x) => x.as_f64().unwrap(),
                        Value::Bool(x) => {
                            if *x {
                                1.0
                            } else {
                                0.0
                            }
                        }
                        Value::String(x) => match x.parse::<f64>() {
                            Ok(x) => x,
                            _ => std::f64::NAN,
                        },
                        _ => std::f64::NAN,
                    };

                    Ok(to_value(num))
                })
                .function("bool", |value| {
                    if value.is_empty() {
                        return Ok(to_value(false));
                    }
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
                    if value.is_empty() {
                        return Ok(to_value("".to_string()));
                    }
                    let v = value.get(0).unwrap();
                    let result: String = match v {
                        Value::Number(x) => {
                            if x.is_f64() {
                                x.as_f64().unwrap().to_string()
                            } else {
                                x.as_i64().unwrap().to_string()
                            }
                        }
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
                    let current_time = chrono::offset::Local::now().time();
                    if extract.is_empty() {
                        return Ok(to_value(current_time.hour()));
                    }

                    let v: String = match extract.get(0).unwrap() {
                        Value::Number(x) => {
                            if x.is_f64() {
                                x.as_f64().unwrap().to_string()
                            } else {
                                x.as_i64().unwrap().to_string()
                            }
                        }
                        Value::Bool(x) => x.to_string(),
                        Value::String(x) => x.to_string(),
                        Value::Array(x) => serde_json::to_string(x).unwrap(),
                        Value::Object(x) => serde_json::to_string(x).unwrap(),
                        _ => String::from("null"),
                    };

                    let result = match v.as_str() {
                        "h" | "hour" | "hours" => current_time.hour(),
                        "m" | "minute" | "minutes" => current_time.minute(),
                        "s" | "second" | "seconds" => current_time.second(),
                        _ => current_time.hour(),
                    };
                    Ok(to_value(result))
                })
                .function("is_match", |value| {
                    if value.len() < 2 {
                        return Ok(to_value(false));
                    }
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
                .value("MIN_INT", std::i64::MIN)
                .value("MAX_INT", std::i64::MAX)
                .value("MAX_FLOAT", std::f64::MAX)
                .value("MIN_FLOAT", std::f64::MIN)
                .value("NAN", std::f64::NAN)
                .value("INFINITY", std::f64::INFINITY)
                .value("NEG_INFINITY", std::f64::NEG_INFINITY)

            // TODO: is_nan(n), is_min_int(n), is_int_max(n), includes(arr)
            // TODO: min(arr), max(arr), abs(n), pow(n, p), sum(arr), reverse(arr), sort(arr), unique(arr)
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

    fn _atoi(s: String) -> i64 {
        let mut item = s
            .trim()
            .split(char::is_whitespace)
            .next()
            .unwrap()
            .split(char::is_alphabetic)
            .next()
            .unwrap();

        let mut end_idx = 0;
        for (pos, c) in item.chars().enumerate() {
            if pos == 0 {
                continue;
            }

            if !c.is_alphanumeric() {
                end_idx = pos;
                break;
            }
        }

        if end_idx > 0 {
            item = &item[0..end_idx];
        }

        let result = item.parse::<i64>();
        match result {
            Ok(v) => return v,
            Err(error) => match error.kind() {
                IntErrorKind::NegOverflow => return std::i64::MIN,
                IntErrorKind::PosOverflow => return std::i64::MAX,
                IntErrorKind::InvalidDigit => {
                    let result = item.parse::<f64>();
                    match result {
                        Ok(v) => return v.round() as i64,
                        _ => return 0,
                    };
                }
                _ => return 0,
            },
        }
    }
}
