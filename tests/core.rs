
#[path = "./../src/core.rs"]
mod core;


#[cfg(test)]
mod tests {
    use eval::{to_value};
    use crate::core::spec::*;

    #[test]
    fn test_eval_literal() {
        let user_spec = Spec::default();

        assert_eq!(user_spec.eval("42"), 42);
        assert_eq!(user_spec.eval("0-42"), -42);
        assert_eq!(user_spec.eval("true"), true);
        assert_eq!(user_spec.eval("false"), false);
        assert_eq!(user_spec.eval("\"42\""), "42");
        assert_eq!(user_spec.eval("'42'"), "42");
        assert_eq!(user_spec.eval("array(42, 42)"), to_value(vec![42; 2]));
        assert_eq!(user_spec.eval("array()"), to_value(vec![0; 0]));
        assert_eq!(user_spec.eval("0..5"), to_value(vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_eval_str_obj() {
        let user_spec = Spec::default();
        assert_eq!(
            user_spec.eval("str(ctx)"),
            "{\"some_var\":\"42\",\"something\":\"true\"}"
        );
    }

    #[test]
    fn test_eval_str_num() {
        let user_spec = Spec::default();
        assert_eq!(user_spec.eval("str(42)"), "42");
    }

    #[test]
    fn test_eval_str_bool() {
        let user_spec = Spec::default();
        assert_eq!(user_spec.eval("str(true)"), "true");
    }

    #[test]
    fn test_eval_str_arr() {
        let user_spec = Spec::default();
        assert_eq!(user_spec.eval("str(array(42, 42))"), to_value("[42,42]"));
    }

    // #[test]
    // fn test_bad_add() {
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
