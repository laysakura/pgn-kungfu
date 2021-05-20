mod enclave;
mod host;

use pgx::*;

pg_module_magic!();

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    use crate::host::integer_avg::IntegerAvgState;

    #[pg_test]
    fn test_integer_avg_state() {
        assert_eq!(
            2,
            IntegerAvgState::default().acc(1).acc(2).acc(3).finalize()
        );
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
