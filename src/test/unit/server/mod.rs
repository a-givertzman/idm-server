use serde::{Deserialize, Serialize};

mod dev_stream_test;
mod device_doc_test;
mod device_info_test;
mod message;

mod fake_select_act;
mod fake_select_req;

mod select_act_test;
mod select_cot_test;
mod select_dev_info_test;
mod select_req_test;

///
/// Fake List of API requiests
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
enum Command {
    Cmd1,
    Cmd2,
    Cmd3,
}
