use pgx::*;

pg_module_magic!();

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PostgresType)]
pub struct IntegerAvgState {
    sum: i32,
    n: i32,
}
impl Default for IntegerAvgState {
    fn default() -> Self {
        Self { sum: 0, n: 0 }
    }
}
impl IntegerAvgState {
    fn acc(&self, v: i32) -> Self {
        Self {
            sum: self.sum + v,
            n: self.n + 1,
        }
    }
    fn finalize(&self) -> i32 {
        self.sum / self.n
    }
}

#[pg_extern]
fn integer_avg_state_func(
    internal_state: IntegerAvgState,
    next_data_value: i32,
) -> IntegerAvgState {
    internal_state.acc(next_data_value)
}

#[pg_extern]
fn integer_avg_final_func(internal_state: IntegerAvgState) -> i32 {
    internal_state.finalize()
}

extension_sql!(
    r#"
    CREATE AGGREGATE MYAVG (integer)
    (
        sfunc = integer_avg_state_func,
        stype = IntegerAvgState,
        finalfunc = integer_avg_final_func,
        initcond = '{"sum": 0, "n": 0}'
    );
    "#
);

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    use crate::IntegerAvgState;

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
