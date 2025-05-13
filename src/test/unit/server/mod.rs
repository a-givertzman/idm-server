use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

mod device_info_test;

mod fake_select_act;
mod fake_select_req;

mod select_act_test;
mod select_cot_test;
mod select_dev_info_test;
mod select_req_test;

///
/// Request kind 1
#[derive(Debug, Serialize, Deserialize)]
struct ReqData(pub String);

///
/// Fake List of API requiests
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
enum Command {
    Cmd1,
    Cmd2,
    Cmd3,
}

///
/// Reply
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Encode, Decode)]
struct Reply {
    id: u32,
    data: String,
    error: Option<String>,
}
