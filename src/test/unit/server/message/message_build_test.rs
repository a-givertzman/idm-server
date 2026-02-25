#[cfg(test)]

mod message {
    use std::{sync::Once, time::{Duration, Instant}};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel};
    use crate::server::{Cot, Field, FieldConf, Terminator, Message, Content};
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
    /// Testing [Message].build
    #[test]
    fn build() {
        DebugSession::new().filter(LogLevel::Debug).init();
        init_once();
        init_each();
        log::debug!("");
        let dbg = Dbg::own("message");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                00, "01234", 4294967291,
                vec![22, 0xff, 0xff, 0xff, 0xfb, Content::String as u8, Cot::Inf as u8, 00, 00, 00, 05, 48, 49, 50, 51, 52],
            ),
            (
                01, "1234 5", 4294967292,
                vec![22, 0xff, 0xff, 0xff, 0xfc, Content::String as u8, Cot::Inf as u8, 00, 00, 00, 06, 49, 50, 51, 52, 32, 53],
            ),
            (
                02, "!@#$%^&*()_+", 4294967293,
                vec![22, 0xff, 0xff, 0xff, 0xfd, Content::String as u8, Cot::Inf as u8, 00, 00, 00, 12, 33, 64, 35, 36, 37, 94, 38, 42, 40, 41, 95, 43],
            ),
            (
                03, r#"QWERTYUIOP{}ASDFGHJKL:"ZXCVBNM<>?""#, 4294967294,
                vec![22, 0xff, 0xff, 0xff, 0xfe, Content::String as u8, Cot::Inf as u8, 00, 00, 00, 34, 81, 87, 69, 82, 84, 89, 85, 73, 79, 80, 123, 125, 65, 83, 68, 70, 71, 72, 74, 75, 76, 58, 34, 90, 88, 67, 86, 66, 78, 77, 60, 62, 63, 34],
            ),
        ];
        let mut message = Message::new(
            &dbg, 
            vec![
                FieldConf::Const(vec![22]),     // Syn
                FieldConf::U32Be,               // ID
                FieldConf::Byte,                // Kind
                FieldConf::Byte,                // Cot
                FieldConf::U32Be,               // Size
                FieldConf::Bytes,               // Payload bytes
            ],
            Terminator::new(),
        );
        for (step, data, id, target) in test_data {
            log::debug!("{} | step: {},  id: {},  kind: {:?},  size: {},  data: {:?}", dbg, step, id, target[1], target[6..].len(), data);
            let data = data.as_bytes().to_vec();
            let time = Instant::now();
            let result = message.build(&[
                Field::Const,
                Field::U32(id),
                Field::Byte(Content::String as u8),
                Field::Byte(Cot::Inf as u8),
                Field::U32(data.len() as u32),
                Field::Bytes(data),
            ]);
            log::debug!("{} | step {step}  elapsed: {:?}",dbg, time.elapsed());
                // data.as_bytes().to_owned().as_mut(), id]);
            assert!(result == target, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
