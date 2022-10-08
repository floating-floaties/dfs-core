#[path = "./../src/core.rs"]
mod core;

#[cfg(test)]
mod eval {
    use std::collections::HashMap;

    use chrono::offset::Utc as Date;
    use chrono::{Datelike, Timelike};
    use resolver::to_value;

    use crate::core::spec::Spec;

    #[test]
    fn global_variables() {
        let user_spec = Spec::default();
        assert_eq!(user_spec.eval("maths.MIN_INT").unwrap(), to_value(i64::MIN));
        assert_eq!(user_spec.eval("maths.MAX_INT").unwrap(), to_value(i64::MAX));
        assert_eq!(user_spec.eval("maths.MAX_FLOAT").unwrap(), to_value(f64::MAX));
        assert_eq!(user_spec.eval("maths.MIN_FLOAT").unwrap(), to_value(f64::MIN));
        assert_eq!(user_spec.eval("NAN").unwrap(), to_value(f64::NAN));
        assert_eq!(user_spec.eval("INFINITY").unwrap(), to_value(f64::INFINITY));
        assert_eq!(
            user_spec.eval("NEG_INFINITY").unwrap(),
            to_value(f64::NEG_INFINITY)
        );
    }

    #[test]
    fn literal() {
        let user_spec = Spec::default();

        assert_eq!(user_spec.eval("42").unwrap(), 42);
        assert_eq!(user_spec.eval("0-42").unwrap(), -42);
        assert_eq!(user_spec.eval("true").unwrap(), true);
        assert_eq!(user_spec.eval("false").unwrap(), false);
        assert_eq!(user_spec.eval("\"42\"").unwrap(), "42");
        assert_eq!(user_spec.eval("'42'").unwrap(), "42");
        assert_eq!(user_spec.eval("array(42, 42)").unwrap(), to_value(vec![42; 2]));
        assert_eq!(user_spec.eval("array()").unwrap(), to_value(vec![0; 0]));
        assert_eq!(user_spec.eval("0..5").unwrap(), to_value(vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn str() {
        let user_spec = Spec::default();
        let expected_ctx_str = "{\"some_var\":\"42\",\"something\":\"true\"}";
        assert_eq!(user_spec.eval("str(ctx)").unwrap(), expected_ctx_str);
        assert_eq!(user_spec.eval("str(42)").unwrap(), "42");
        assert_eq!(user_spec.eval("str(42.42)").unwrap(), "42.42");
        assert_eq!(user_spec.eval("str(true)").unwrap(), "true");
        assert_eq!(user_spec.eval("str(array(42, 42))").unwrap(), to_value("[42,42]"));
        assert_eq!(user_spec.eval("str(array())").unwrap(), to_value("[]"));
        assert_eq!(user_spec.eval("str(null)").unwrap(), to_value("null"));
    }

    #[test]
    fn bool() {
        let mut user_spec = Spec::default();

        assert_eq!(user_spec.eval("bool(ctx)").unwrap(), true);
        user_spec.context = HashMap::new();
        assert_eq!(user_spec.eval("bool(ctx)").unwrap(), false);

        assert_eq!(user_spec.eval("bool(1)").unwrap(), true);
        assert_eq!(user_spec.eval("bool(1.0)").unwrap(), true);
        assert_eq!(user_spec.eval("bool(0)").unwrap(), false);
        assert_eq!(user_spec.eval("bool(0.0)").unwrap(), false);
        assert_eq!(user_spec.eval("bool(true)").unwrap(), true);
        assert_eq!(user_spec.eval("bool(false)").unwrap(), false);

        assert_eq!(user_spec.eval("bool(42)").unwrap(), true);
        assert_eq!(user_spec.eval("bool(42.42)").unwrap(), true);
        assert_eq!(user_spec.eval("bool(0-42)").unwrap(), true);
        assert_eq!(user_spec.eval("bool(0-42.42)").unwrap(), true);

        assert_eq!(user_spec.eval("bool('')").unwrap(), false);
        assert_eq!(user_spec.eval("bool(\"\")").unwrap(), false);
        assert_eq!(user_spec.eval("bool('42')").unwrap(), true);
        assert_eq!(user_spec.eval("bool(\"42\")").unwrap(), true);

        assert_eq!(user_spec.eval("bool(array(42, 42))").unwrap(), true);
        assert_eq!(user_spec.eval("bool(array())").unwrap(), false);
        assert_eq!(user_spec.eval("bool(0..42)").unwrap(), true);
        assert_eq!(user_spec.eval("bool(0..0)").unwrap(), false);
        assert_eq!(user_spec.eval("bool(null)").unwrap(), false);
    }

    #[test]
    fn float() {
        let user_spec = Spec::default();
        assert_eq!(user_spec.eval("float(42)").unwrap(), 42.0);
        assert_eq!(user_spec.eval("float(42.42)").unwrap(), 42.42);
        assert_eq!(user_spec.eval("float('42.42')").unwrap(), 42.42);
        assert_eq!(user_spec.eval("float('42')").unwrap(), 42.0);
        assert_eq!(user_spec.eval("float(true)").unwrap(), 1.0);
        assert_eq!(user_spec.eval("float(false)").unwrap(), 0.0);
        assert_eq!(user_spec.eval("float('')").unwrap(), to_value(f64::NAN));
        assert_eq!(
            user_spec.eval("float('not a num')").unwrap(),
            to_value(f64::NAN)
        );
        assert_eq!(user_spec.eval("float(ctx)").unwrap(), to_value(f64::NAN));
        assert_eq!(
            user_spec.eval("float(array(42, 42))").unwrap(),
            to_value(f64::NAN)
        );
        assert_eq!(user_spec.eval("float(0..42)").unwrap(), to_value(f64::NAN));
        assert_eq!(user_spec.eval("float(null)").unwrap(), to_value(f64::NAN));
    }

    #[test]
    fn int() {
        let user_spec = Spec::default();
        assert_eq!(user_spec.eval("int(42)").unwrap(), 42);
        assert_eq!(user_spec.eval("int(42.42)").unwrap(), 42);
        assert_eq!(user_spec.eval("int('42.42')").unwrap(), 42);
        assert_eq!(user_spec.eval("int('42')").unwrap(), 42);
        assert_eq!(user_spec.eval("int(true)").unwrap(), 1);
        assert_eq!(user_spec.eval("int(false)").unwrap(), 0);
        assert_eq!(user_spec.eval("int('')").unwrap(), 0);
        assert_eq!(user_spec.eval("int('not a num')").unwrap(), 0);
        assert_eq!(user_spec.eval("int(ctx)").unwrap(), 0);
        assert_eq!(user_spec.eval("int(array(42, 42))").unwrap(), 0);
        assert_eq!(user_spec.eval("int(0..42)").unwrap(), 0);
        assert_eq!(user_spec.eval("int(null)").unwrap(), 0);
    }

    #[test]
    fn day() {
        let user_spec = Spec::default();
        let date = Date::now().date();
        let day = date.day();

        assert_eq!(user_spec.eval("get_day()").unwrap(), day);
        assert_eq!(user_spec.eval("get_day('_')").unwrap(), day);
    }

    #[test]
    fn month() {
        let user_spec = Spec::default();
        let date = Date::now().date();
        let month = date.month();

        assert_eq!(user_spec.eval("get_month()").unwrap(), month);
        assert_eq!(user_spec.eval("get_month('_')").unwrap(), month);
    }

    #[test]
    fn year() {
        let user_spec = Spec::default();
        let date = Date::now().date();
        let year = date.year();
        assert_eq!(user_spec.eval("get_year()").unwrap(), year);
        assert_eq!(user_spec.eval("get_year('_')").unwrap(), year);
    }

    #[test]
    fn weekday() {
        let user_spec = Spec::default();
        let weekday_num = Date::now().weekday().number_from_monday();
        assert_eq!(user_spec.eval("get_weekday('_')").unwrap(), weekday_num);
        assert_eq!(user_spec.eval("is_weekday('_')").unwrap(), weekday_num < 6);

        assert_eq!(user_spec.eval("get_weekday()").unwrap(), weekday_num);
        assert_eq!(user_spec.eval("is_weekday()").unwrap(), weekday_num < 6);
    }

    #[test]
    fn time() {
        let user_spec = Spec::default();
        assert_eq!(user_spec.eval("get_time('_', 'h')").unwrap(), Date::now().time().hour());
        assert_eq!(user_spec.eval("get_time('_', 'm')").unwrap(), Date::now().time().minute());
        assert_eq!(user_spec.eval("get_time('_', 's')").unwrap(), Date::now().time().second());

        assert_eq!(user_spec.eval("get_time('_', 'hour')").unwrap(), Date::now().time().hour());
        assert_eq!(
            user_spec.eval("get_time('_', 'minute')").unwrap(),
            Date::now().time().minute()
        );
        assert_eq!(
            user_spec.eval("get_time('_', 'second')").unwrap(),
            Date::now().time().second()
        );

        assert_eq!(user_spec.eval("get_time('_', 'hours')").unwrap(), Date::now().time().hour());
        assert_eq!(
            user_spec.eval("get_time('_', 'minutes')").unwrap(),
            Date::now().time().minute()
        );
        assert_eq!(
            user_spec.eval("get_time('_', 'seconds')").unwrap(),
            Date::now().time().second()
        );
    }
}
