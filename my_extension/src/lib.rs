use pgx::*;

pg_module_magic!();

#[pg_extern]
fn integer_add(internal_state: i32, next_data_value: i32) -> i32 {
    internal_state + next_data_value
}

extension_sql!(
    r#"
    CREATE AGGREGATE MYSUM (integer)
    (
        sfunc = integer_add,
        stype = integer,
        initcond = '0'
    );
    "#
);

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_integer_add() {
        assert_eq!(42, crate::integer_add(-1, 43));
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
