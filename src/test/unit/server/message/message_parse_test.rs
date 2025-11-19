#[cfg(test)]

mod message {
    use std::{sync::Once, time::{Duration, Instant}};
    use sal_core::{dbg::Dbg, error::Error};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel};
    use crate::server::{Cot, FieldConf, FieldId, Terminator, FindField, FixedField, Message, Content, SizedField};
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// 
    struct FieldSize(pub u32);
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// Returns bytes from `data` and `id`
    fn to_bytes(data: &str, id: u32, cot: Cot) -> Vec<u8> {
        let data = data.as_bytes();
        let size = data.len() as u32;
        [
            &[22],
            FieldId(id).to_be_bytes().as_slice(),
            &[Content::String as u8],
            &[cot as u8],
            size.to_be_bytes().as_slice(),
            data,
        ].concat()
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing [Message].parse
    #[test]
    fn parse() {
        DebugSession::new().filter(LogLevel::Debug).init();
        init_once();
        init_each();
        log::debug!("");
        let dbg = Dbg::own("message_parse");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                00, vec![
                    to_bytes("123", 4294967291, Cot::Inf),
                ],
                FieldId(4294967291), Content::String, Cot::Inf, FieldSize(3), vec![49, 50, 51],
            ),
            (
                01, vec![
                    vec![000, 001, 002, 002],
                    to_bytes("23456", 4294967292, Cot::Inf)[..3].to_vec(),
                    to_bytes("23456", 4294967292, Cot::Inf)[3..].to_vec(),
                ],
                FieldId(4294967292), Content::String, Cot::Inf, FieldSize(5), vec![50, 51, 52, 53, 54],
            ),
            (
                02, vec![
                    vec![000, 001, 002, 002],
                    [&[003, 004], &to_bytes("23456", 4294967293, Cot::Inf)[..4]].concat(),
                    to_bytes("23456", 4294967293, Cot::Inf)[4..].to_vec(),
                ],
                FieldId(4294967293), Content::String, Cot::Inf, FieldSize(5), vec![50, 51, 52, 53, 54],
            ),
            (
                03, vec![
                    vec![011, 012, 013, 014],
                    to_bytes("2345678", 4294967294, Cot::Inf)[..3].to_vec(),
                    to_bytes("2345678", 4294967294, Cot::Inf)[3..4].to_vec(),
                    to_bytes("2345678", 4294967294, Cot::Inf)[4..6].to_vec(),
                    to_bytes("2345678", 4294967294, Cot::Inf)[6..].to_vec(),
                ],
                FieldId(4294967294), Content::String, Cot::Inf, FieldSize(7), vec![50, 51, 52, 53, 54, 55, 56],
            ),
            (
                04, vec![
                    vec![111, 112, 113, 114],
                    to_bytes("234567890", 4294967295, Cot::Inf)[..3].to_vec(),
                    to_bytes("234567890", 4294967295, Cot::Inf)[3..4].to_vec(),
                    to_bytes("234567890", 4294967295, Cot::Inf)[4..7].to_vec(),
                    to_bytes("234567890", 4294967295, Cot::Inf)[7..16].to_vec(),
                    to_bytes("234567890", 4294967295, Cot::Inf)[16..].to_vec(),
                ],
                FieldId(4294967295), Content::String, Cot::Inf, FieldSize(9), vec![50, 51, 52, 53, 54, 55, 56, 57, 48],
            ),
        ];
        let mut message: Message<(((((((), ()), ()), FieldId), Content), Cot), u32), Vec<u8>> = Message::new(
            &dbg, // Start |  Id   | Kind | Cot  |  Size  | Data
            vec![
                FieldConf::Const(vec![22]),     // Syn
                FieldConf::U32Be,               // ID
                FieldConf::Byte,                // Kind
                FieldConf::Byte,                // Cot
                FieldConf::U32Be,               // Size
                FieldConf::Bytes,               // Payload bytes
            ],
            SizedField::new(
                &dbg,
                |_, size| *size as usize,
                |_, bytes| {
                    Ok(bytes.to_vec())
                },
                FixedField::new(        // Size | u32
                    &dbg,
                    4,
                    |dbg, bytes| {
                        match bytes.try_into() {
                            Ok(bytes) => Ok(u32::from_be_bytes(bytes)),
                            Err(err) => panic!("{}", Error::new(dbg, "Id::from_bytes").pass_with(format!("Can't parse 'Size' u32 filed from bytes {:?}", bytes), format!("{err}"))),
                        }
                    },
                    FixedField::new(        // Cot | u8
                        &dbg,
                        1,
                        |dbg, bytes| {
                            match Cot::from_be_bytes(bytes) {
                                Ok(cot) => Ok(cot),
                                Err(err) => panic!("{}", Error::new(dbg, "Cot::from_bytes").pass_with("Can't parse 'Cot' u8 field", format!("{err}"))),
                            }
                        },
                        FixedField::new(        // Kind | u8
                            &dbg,
                            1,
                            |dbg, bytes| {
                                match Content::try_from(bytes) {
                                    Ok(kind) => Ok(kind),
                                    Err(err) => panic!("{}", Error::new(dbg, "Kind::from_bytes").pass_with("Can't parse 'Kind' u8 field", format!("{err}"))),
                                }
                            },
                            FixedField::new(        // ID | u32
                                &dbg,
                                4,
                                |dbg, bytes| {
                                    match FieldId::from_be_bytes(bytes) {
                                        Ok(id) => Ok(id),
                                        Err(err) => panic!("{}", Error::new(dbg, "Id::from_bytes").pass_with("Can't parse 'ID' u32 filed", format!("{err}"))),
                                    }
                                },
                                FindField::new(        // SYN | u8
                                    &dbg,
                                    1,
                                    |_, bytes| {
                                        match bytes {
                                            [22] | [22, ..] => Ok(Some(())),
                                            [_] | [_, ..] => Ok(None),
                                            [] => Ok(None)
                                        }
                                    },
                                    Terminator::new(),
                                ),
                            ),
                        ),
                    ),
                ),
            )
        );
        for (step, messages, target_id, target_kind, target_cot, FieldSize(target_size), target_bytes) in test_data {
            let mut result_bytes = vec![];
            let mut time = Instant::now();
            for bytes in messages {
                match message.parse(bytes) {
                    Ok((((((_, id), kind), cot), size), bytes)) => {
                        log::debug!("{} | step: {},  id: {:?},  kind: {:?},  size: {:?},  bytes: {:?}", dbg, step, id, kind, size, bytes);
                        let result = id;
                        assert!(result == target_id, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_id);
                        let result = kind;
                        assert!(result == target_kind, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_kind);
                        let result = cot;
                        assert!(result == target_cot, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_cot);
                        let result = size;
                        assert!(result == target_size, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_size);
                        result_bytes.extend(bytes);
                        log::debug!("{} | step {step}  elapsed: {:?}",dbg, time.elapsed());
                        time = Instant::now()
                    }
                    Err(err) => {
                        log::trace!("{} | {}",dbg, err);
                    }
                }
            }
            // message.reset();
            assert!(result_bytes == target_bytes, "step: {} \nresult: {:?}\ntarget: {:?}", step, result_bytes, target_bytes);
        }
        test_duration.exit();
    }
    ///
    /// Testing such `Message.parse`
    #[test]
    fn parse_advanced() {
        DebugSession::new().filter(LogLevel::Debug).init();
        init_once();
        init_each();
        log::debug!("");
        let dbg = Dbg::own("parse_advanced");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let field_kind = Content::String;
        let test_data = [
            (
                00, vec![
                    to_bytes("123", 4294967291, Cot::Act),
                ],
                (FieldId(4294967291), field_kind.clone(), Cot::Act, FieldSize(3), "123".as_bytes().to_vec()),
            ),
            (
                01, vec![
                    [10, 11, 12, 13].to_vec(),
                    to_bytes("12345", 4294967292, Cot::Act)[..1].to_vec(),
                    to_bytes("12345", 4294967292, Cot::Act)[1..].to_vec(),
                ],
                (FieldId(4294967292), field_kind.clone(), Cot::Act, FieldSize(5), "12345".as_bytes().to_vec()),
            ),
            (
                02, vec![
                    to_bytes("2345", 4294967293, Cot::Act)[..2].to_vec(),
                    to_bytes("2345", 4294967293, Cot::Act)[2..].to_vec(),
                    [20, 21, 23, 24].to_vec(),
                ],
                (FieldId(4294967293), field_kind.clone(), Cot::Act, FieldSize(4), "2345".as_bytes().to_vec()),
            ),
            (
                03, vec![
                    [31, 32, 33, 34].to_vec(),
                    to_bytes("3456", 4294967294, Cot::Act)[..3].to_vec(),
                    to_bytes("3456", 4294967294, Cot::Act)[3..].to_vec(),
                    [35, 36, 37, 38].to_vec(),
                ],
                (FieldId(4294967294), field_kind.clone(), Cot::Act, FieldSize(4), "3456".as_bytes().to_vec()),
            ),
            (
                04, vec![
                    to_bytes("456", 4294967295, Cot::Act)[..4].to_vec(),
                    to_bytes("456", 4294967295, Cot::Act)[4..].to_vec(),
                ],
                (FieldId(4294967295), field_kind.clone(), Cot::Act, FieldSize(3), "456".as_bytes().to_vec()),
            ),
            (
                05, vec![
                    to_bytes("56789", 4294967281, Cot::Act)[..5].to_vec(),
                    to_bytes("56789", 4294967281, Cot::Act)[5..].to_vec(),
                ],
                (FieldId(4294967281), field_kind.clone(), Cot::Act, FieldSize(5), "56789".as_bytes().to_vec()),
            ),
            (
                06, vec![
                    to_bytes("67890", 4294967282, Cot::Act)[..6].to_vec(),
                    to_bytes("67890", 4294967282, Cot::Act)[6..].to_vec(),
                ],
                (FieldId(4294967282), field_kind.clone(), Cot::Act, FieldSize(5), "67890".as_bytes().to_vec()),
            ),
            (
                07, vec![
                    to_bytes("78901", 4294967283, Cot::Act)[..7].to_vec(),
                    to_bytes("78901", 4294967283, Cot::Act)[7..].to_vec(),
                ],
                (FieldId(4294967283), field_kind.clone(), Cot::Act, FieldSize(5), "78901".as_bytes().to_vec()),
            ),
            (
                08, vec![
                    [80, 81, 82, 83].to_vec(),
                    to_bytes("1234567890", 4294967284, Cot::Act)[ ..2].to_vec(),
                    to_bytes("1234567890", 4294967284, Cot::Act)[2..5].to_vec(),
                    to_bytes("1234567890", 4294967284, Cot::Act)[5..7].to_vec(),
                    to_bytes("1234567890", 4294967284, Cot::Act)[7..9].to_vec(),
                    to_bytes("1234567890", 4294967284, Cot::Act)[9.. ].to_vec(),
                ],
                (FieldId(4294967284), field_kind.clone(), Cot::Act, FieldSize(10), "1234567890".as_bytes().to_vec()),
            ),
            (
                09, vec![
                    [[34, 36].into(), to_bytes("123N", 4294967285, Cot::Act), [78, 22].into()].concat(),
                ],
                (FieldId(4294967285), field_kind.clone(), Cot::Act, FieldSize(4), "123N".as_bytes().to_vec()),
            ),
        ];
        let mut message: Message<(((((((), ()), ()), FieldId), Content), Cot), u32), Vec<u8>> = Message::new(
            &dbg, // Start |  Id   | Kind | Cot  |  Size  | Data
            vec![
                FieldConf::Const(vec![22]),     // Syn
                FieldConf::U32Be,               // ID
                FieldConf::Byte,                // Kind
                FieldConf::Byte,                // Cot
                FieldConf::U32Be,               // Size
                FieldConf::Bytes,               // Payload bytes
            ],
            SizedField::new(
                &dbg,
                |_, size| *size as usize,
                |_, bytes| {
                    Ok(bytes.to_vec())
                },
                FixedField::new(        // Size | u32
                    &dbg,
                    4,
                    |dbg, bytes| {
                        match bytes.try_into() {
                            Ok(bytes) => Ok(u32::from_be_bytes(bytes)),
                            Err(err) => panic!("{}", Error::new(dbg, "Id::from_bytes").pass_with(format!("Can't parse 'Size' u32 filed from bytes {:?}", bytes), format!("{err}"))),
                        }
                    },
                    FixedField::new(        // Cot | u8
                        &dbg,
                        1,
                        |dbg, bytes| {
                            match Cot::from_be_bytes(bytes) {
                                Ok(cot) => Ok(cot),
                                Err(err) => panic!("{}", Error::new(dbg, "Cot::from_bytes").pass_with("Can't parse 'Cot' u8 field", format!("{err}"))),
                            }
                        },
                        FixedField::new(        // Kind | u8
                            &dbg,
                            1,
                            |dbg, bytes| {
                                match Content::try_from(bytes) {
                                    Ok(kind) => Ok(kind),
                                    Err(err) => panic!("{}", Error::new(dbg, "Kind::from_bytes").pass_with("Can't parse 'Kind' u8 field", format!("{err}"))),
                                }
                            },
                            FixedField::new(        // ID | u32
                                &dbg,
                                4,
                                |dbg, bytes| {
                                    match FieldId::from_be_bytes(bytes) {
                                        Ok(id) => Ok(id),
                                        Err(err) => panic!("{}", Error::new(dbg, "Id::from_bytes").pass_with("Can't parse 'ID' u32 filed", format!("{err}"))),
                                    }
                                },
                                FindField::new(        // SYN | u8
                                    &dbg,
                                    1,
                                    |_, bytes| {
                                        match bytes {
                                            [22] | [22, ..] => Ok(Some(())),
                                            [_] | [_, ..] => Ok(None),
                                            [] => Ok(None)
                                        }
                                    },
                                    Terminator::new(),
                                ),
                            ),
                        ),
                    ),
                ),
            )
        );
        for (step, messages, (target_id, target_kind, target_cot, FieldSize(target_size), target_bytes)) in test_data {
            let mut result_data = None;
            let mut time = Instant::now();
            for bytes in messages {
                match message.parse(bytes) {
                    Ok((((((_, id), kind), cot), size), bytes)) => {
                        log::debug!("{} | step: {},  id: {:?},  kind: {:?},  size: {:?},  data: {:?}", dbg, step, id, kind, size, bytes);
                        let result = id;
                        assert!(result == target_id, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_id);
                        let result = kind;
                        assert!(result == target_kind, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_kind);
                        let result = cot;
                        assert!(result == target_cot, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_cot);
                        let result = size;
                        assert!(result == target_size, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_size);
                        let result = bytes;
                        assert!(result == target_bytes, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_bytes);
                        result_data = Some(result);
                        log::debug!("{} | step {step}  elapsed: {:?}",dbg, time.elapsed());
                        time = Instant::now()
                    }
                    Err(err) => {
                        log::trace!("{} | {}",dbg, err);
                    }
                }
            }
            assert!(result_data == Some(target_bytes.clone()), "step: {} \nresult: {:?}\ntarget: {:?}", step, result_data, Some(target_bytes));
        }
        test_duration.exit();
    }
}
