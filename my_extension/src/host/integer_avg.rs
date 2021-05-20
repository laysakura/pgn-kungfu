use pgx::*;
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
    pub fn acc(&self, v: i32) -> Self {
        Self {
            sum: self.sum + v,
            n: self.n + 1,
        }
    }
    pub fn finalize(&self) -> i32 {
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
