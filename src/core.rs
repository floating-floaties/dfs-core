#![allow(dead_code)]

#[path = "./utils/utilities.rs"]
mod utils;

pub mod spec {
    use std::collections::{BTreeMap, HashMap};

    use resolver;
    use serde::{Deserialize, Serialize};
    use eval_utility::eval_wrapper::{ExprWrapper, EvalConfig};

    pub mod web {
        use crate::core::spec::Spec;

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct ResultType {
            pub value: resolver::Value,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct ConditionRequest {
            pub spec: Spec,
            pub condition: String,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct ConditionResponse {
            pub message: String,
            pub result: Option<ResultType>,
            pub error: bool,
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Case {
        pub condition: String,
        pub reply: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Dialog {
        pub intent: String,
        pub cases: Vec<Case>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Spec {
        pub intents: Vec<String>,
        pub context: HashMap<String, String>,
        pub system: HashMap<String, String>,
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
            system: HashMap<String, String>,
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
                system,
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

            let mut system = HashMap::<String, String>::new();
            system.insert("timezone".to_owned(), "US/Eastern".to_owned());
            Self::new(intents, dialogs, context, system)
        }

        pub fn expr(&self, expression: String) -> ExprWrapper {
            ExprWrapper::new(expression)
                .value("ctx", &self.context)
                .value("sys", &self.system)
                .config(EvalConfig {
                    include_maths: true,
                    include_regex: true,
                    include_datetime: true,
                    include_cast: true,
                })
                .init()
        }

        pub fn eval<S: AsRef<str>>(&self, expression: S) -> Result<resolver::Value, String> {
            let str_like = expression.as_ref().to_owned();
            let result = self.expr(str_like).exec();
            match result {
                Ok(result) => Ok(result),
                Err(error) => {
                    let message = format!(
                        "Failed to parse expression: \"{}\"; {:?}",
                        expression.as_ref().to_owned(),
                        error,
                    );
                    Err(message)
                }
            }
        }

        pub fn format_eval_for_response<S: AsRef<str>>(
            &self,
            expression: S,
        ) -> Result<web::ResultType, String> {
            let evaluated_expression = self.eval(expression);

            match evaluated_expression {
                Ok(value) => Ok(web::ResultType {
                    value
                }),
                Err(message) => Err(message),
            }
        }

        pub fn from_yaml(content: &str) -> Self {
            serde_yaml::from_str(content).unwrap()
        }

        pub fn from_json(content: &str) -> Self {
            serde_json::from_str(content).unwrap()
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
