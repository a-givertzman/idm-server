#[cfg(test)]

mod parse_id {
    use std::{sync::Once, time::Duration};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel};

    use crate::server::Field;
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing [Field].to_be_bytes()
    #[test]
    fn to_be_bytes() {
        DebugSession::new().filter(LogLevel::Debug).init();
        init_once();
        init_each();
        log::debug!("");
        let dbg = Dbg::own("Field.to_be_bytes");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (00, Field::Const, vec![]),
            (01, Field::Byte(254), vec![254]),
            (02, Field::Bytes(vec![1, 23, 255, 64]), vec![1, 23, 255, 64]),
            (03, Field::F32(3454.234325), 3454.234325f32.to_be_bytes().to_vec()),
            (04, Field::F32(-3454.234325), (-3454.234325f32).to_be_bytes().to_vec()),
            (05, Field::F32(f32::MAX), f32::MAX.to_be_bytes().to_vec()),
            (06, Field::F32(-f32::MAX), (-f32::MAX).to_be_bytes().to_vec()),
            (07, Field::F32(f32::MIN), f32::MIN.to_be_bytes().to_vec()),
            (08, Field::F32(-f32::MIN), (-f32::MIN).to_be_bytes().to_vec()),
            (09, Field::F64(3454.234325), 3454.234325f64.to_be_bytes().to_vec()),
            (10, Field::F64(-3454.234325), (-3454.234325f64).to_be_bytes().to_vec()),
            (11, Field::F64(f64::MAX), f64::MAX.to_be_bytes().to_vec()),
            (12, Field::F64(-f64::MAX), (-f64::MAX).to_be_bytes().to_vec()),
            (13, Field::F64(f64::MIN), f64::MIN.to_be_bytes().to_vec()),
            (14, Field::F64(-f64::MIN), (-f64::MIN).to_be_bytes().to_vec()),
            (02, Field::I128(12325564), 12325564i128.to_be_bytes().to_vec()),
            (02, Field::I128(-12325564), (-12325564i128).to_be_bytes().to_vec()),
            (02, Field::I16(25564), 25564i16.to_be_bytes().to_vec()),
            (02, Field::I16(-25564), (-25564i16).to_be_bytes().to_vec()),
            (02, Field::I32(2325564), 2325564i32.to_be_bytes().to_vec()),
            (02, Field::I32(-2325564), (-2325564i32).to_be_bytes().to_vec()),
            (02, Field::I64(12325564), 12325564i64.to_be_bytes().to_vec()),
            (02, Field::I64(-12325564), (-12325564i64).to_be_bytes().to_vec()),
            (02, Field::I8(127), 127i8.to_be_bytes().to_vec()),
            (02, Field::I8(-128), (-128i8).to_be_bytes().to_vec()),
            (02, Field::Json(format!("asejfhow3874htlfgjsdbgliwu34hytl;iwu4ehagb.kjas")), "asejfhow3874htlfgjsdbgliwu34hytl;iwu4ehagb.kjas".as_bytes().to_vec()),
            (02, Field::String(format!("asejfhow3874htlfgjsdbgliwu34hytl;iwu4ehagb.kjas")), "asejfhow3874htlfgjsdbgliwu34hytl;iwu4ehagb.kjas".as_bytes().to_vec()),
            (02, Field::U128(12325564), 12325564u128.to_be_bytes().to_vec()),
            (02, Field::U16(25564), 25564u16.to_be_bytes().to_vec()),
            (02, Field::U32(2325564), 2325564u32.to_be_bytes().to_vec()),
            (02, Field::U64(12325564), 12325564u64.to_be_bytes().to_vec()),
        ];
        for (step, field, target) in test_data {
            let result = field.to_be_bytes();
            log::debug!("{} | step: {},  id: {:?},  bytes: {:?}", dbg, step, field, result);
            assert!(result == target, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
