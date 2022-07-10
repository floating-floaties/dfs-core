#[path = "./../src/core.rs"]
mod core;

#[cfg(test)]
mod eval {
    use chrono::{Datelike, Timelike};
    use std::collections::HashMap;

    use crate::core::spec::*;
    use eval::to_value;

    #[test]
    fn global_variables() {
        let user_spec = Spec::default();
        assert_eq!(user_spec.eval("MIN_INT"), to_value(std::i64::MIN));
        assert_eq!(user_spec.eval("MAX_INT"), to_value(std::i64::MAX));
        assert_eq!(user_spec.eval("MAX_FLOAT"), to_value(std::f64::MAX));
        assert_eq!(user_spec.eval("MIN_FLOAT"), to_value(std::f64::MIN));
        assert_eq!(user_spec.eval("NAN"), to_value(std::f64::NAN));
        assert_eq!(user_spec.eval("INFINITY"), to_value(std::f64::INFINITY));
        assert_eq!(user_spec.eval("NEG_INFINITY"), to_value(std::f64::NEG_INFINITY));
    }

    #[test]
    fn literal() {
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
    fn str() {
        let user_spec = Spec::default();
        let expected_ctx_str = "{\"some_var\":\"42\",\"something\":\"true\"}";
        assert_eq!(user_spec.eval("str(ctx)"), expected_ctx_str);
        assert_eq!(user_spec.eval("str(42)"), "42");
        assert_eq!(user_spec.eval("str(42.42)"), "42.42");
        assert_eq!(user_spec.eval("str(true)"), "true");
        assert_eq!(user_spec.eval("str(array(42, 42))"), to_value("[42,42]"));
        assert_eq!(user_spec.eval("str(array())"), to_value("[]"));
        assert_eq!(user_spec.eval("str(null)"), to_value("null"));
    }

    #[test]
    fn bool() {
        let mut user_spec = Spec::default();

        assert_eq!(user_spec.eval("bool(ctx)"), true);
        user_spec.context = HashMap::new();
        assert_eq!(user_spec.eval("bool(ctx)"), false);

        assert_eq!(user_spec.eval("bool(1)"), true);
        assert_eq!(user_spec.eval("bool(1.0)"), true);
        assert_eq!(user_spec.eval("bool(0)"), false);
        assert_eq!(user_spec.eval("bool(0.0)"), false);
        assert_eq!(user_spec.eval("bool(true)"), true);
        assert_eq!(user_spec.eval("bool(false)"), false);

        assert_eq!(user_spec.eval("bool(42)"), true);
        assert_eq!(user_spec.eval("bool(42.42)"), true);
        assert_eq!(user_spec.eval("bool(0-42)"), true);
        assert_eq!(user_spec.eval("bool(0-42.42)"), true);

        assert_eq!(user_spec.eval("bool('')"), false);
        assert_eq!(user_spec.eval("bool(\"\")"), false);
        assert_eq!(user_spec.eval("bool('42')"), true);
        assert_eq!(user_spec.eval("bool(\"42\")"), true);

        assert_eq!(user_spec.eval("bool(array(42, 42))"), true);
        assert_eq!(user_spec.eval("bool(array())"), false);
        assert_eq!(user_spec.eval("bool(0..42)"), true);
        assert_eq!(user_spec.eval("bool(0..0)"), false);
        assert_eq!(user_spec.eval("bool(null)"), false);
    }

    #[test]
    fn float() {
        let user_spec = Spec::default();
        assert_eq!(user_spec.eval("float(42)"), 42.0);
        assert_eq!(user_spec.eval("float(42.42)"), 42.42);
        assert_eq!(user_spec.eval("float('42.42')"), 42.42);
        assert_eq!(user_spec.eval("float('42')"), 42.0);
        assert_eq!(user_spec.eval("float(true)"), 1.0);
        assert_eq!(user_spec.eval("float(false)"), 0.0);
        assert_eq!(user_spec.eval("float('')"), to_value(std::f64::NAN));
        assert_eq!(
            user_spec.eval("float('not a num')"),
            to_value(std::f64::NAN)
        );
        assert_eq!(user_spec.eval("float(ctx)"), to_value(std::f64::NAN));
        assert_eq!(
            user_spec.eval("float(array(42, 42))"),
            to_value(std::f64::NAN)
        );
        assert_eq!(user_spec.eval("float(0..42)"), to_value(std::f64::NAN));
        assert_eq!(user_spec.eval("float(null)"), to_value(std::f64::NAN));
    }

    #[test]
    fn int() {
        let user_spec = Spec::default();
        assert_eq!(user_spec.eval("int(42)"), 42);
        assert_eq!(user_spec.eval("int(42.42)"), 42);
        assert_eq!(user_spec.eval("int('42.42')"), 42);
        assert_eq!(user_spec.eval("int('42')"), 42);
        assert_eq!(user_spec.eval("int(true)"), 1);
        assert_eq!(user_spec.eval("int(false)"), 0);
        assert_eq!(user_spec.eval("int('')"), 0);
        assert_eq!(user_spec.eval("int('not a num')"), 0);
        assert_eq!(user_spec.eval("int(ctx)"), 0);
        assert_eq!(user_spec.eval("int(array(42, 42))"), 0);
        assert_eq!(user_spec.eval("int(0..42)"), 0);
        assert_eq!(user_spec.eval("int(null)"), 0);
    }

    #[test]
    fn day() {
        let user_spec = Spec::default();
        let date = chrono::offset::Local::now().date();
        let day = date.day();

        assert_eq!(user_spec.eval("day()"), day);
        assert_eq!(user_spec.eval("day(with, args)"), day);
    }

    #[test]
    fn month() {
        let user_spec = Spec::default();
        let date = chrono::offset::Local::now().date();
        let month = date.month();

        assert_eq!(user_spec.eval("month()"), month);
        assert_eq!(user_spec.eval("month(with, args)"), month);

    }

    #[test]
    fn year() {
        let user_spec = Spec::default();
        let date = chrono::offset::Local::now().date();
        let year = date.year();
        assert_eq!(user_spec.eval("year()"), year);
        assert_eq!(user_spec.eval("year(with, args)"), year);
    }

    #[test]
    fn weekday() {
        let user_spec = Spec::default();
        let weekday_num = chrono::offset::Local::now().weekday().number_from_monday();
        assert_eq!(user_spec.eval("weekday()"), weekday_num);
        assert_eq!(user_spec.eval("is_weekday()"), weekday_num < 6);

        assert_eq!(user_spec.eval("weekday(with, args)"), weekday_num);
        assert_eq!(user_spec.eval("is_weekday(with, args)"), weekday_num < 6);
    }

    #[test]
    fn time() {
        let user_spec = Spec::default();
        assert_eq!(
            user_spec.eval("time('h')"),
            chrono::offset::Local::now().time().hour()
        );
        assert_eq!(
            user_spec.eval("time('m')"),
            chrono::offset::Local::now().time().minute()
        );
        assert_eq!(
            user_spec.eval("time('s')"),
            chrono::offset::Local::now().time().second()
        );

        assert_eq!(
            user_spec.eval("time('hour')"),
            chrono::offset::Local::now().time().hour()
        );
        assert_eq!(
            user_spec.eval("time('minute')"),
            chrono::offset::Local::now().time().minute()
        );
        assert_eq!(
            user_spec.eval("time('second')"),
            chrono::offset::Local::now().time().second()
        );

        assert_eq!(
            user_spec.eval("time('hours')"),
            chrono::offset::Local::now().time().hour()
        );
        assert_eq!(
            user_spec.eval("time('minutes')"),
            chrono::offset::Local::now().time().minute()
        );
        assert_eq!(
            user_spec.eval("time('seconds')"),
            chrono::offset::Local::now().time().second()
        );
    }
}
