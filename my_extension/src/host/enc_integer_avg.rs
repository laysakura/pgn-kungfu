mod error;
mod typ;

pub use typ::EncInteger;

pub(crate) use error::DecryptError;

use pgx::*;

extension_sql!(
    r#"
    CREATE AGGREGATE ENCAVG (EncInteger)
    (
        sfunc = enc_integer_avg_state_func,
        stype = IntegerAvgState,
        finalfunc = enc_integer_avg_final_func,
        initcond = '{"sum": 0, "n": 0}'
    );
    "#
);
