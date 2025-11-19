use sal_core::error::Error;

///
/// Identifier of API `Query`'s
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
#[repr(u32)]
pub enum QueryId {
    DeviceStream = Self::DEVICE_STREAM,
    DeviceInfo   = Self::DEVICE_INFO,
    DeviceDoc    = Self::DEVICE_DOC,
}
//
//
impl QueryId {
    const DEVICE_STREAM : u32 = 0x16;
    const DEVICE_INFO   : u32 = 0x20;
    const DEVICE_DOC    : u32 = 0x24;
    ///
    /// Returns [Query] from `bytes`
    pub fn from_be_bytes(bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            [b0, b1, b2, b3] | [b0, b1, b2, b3, ..] => {
                match u32::from_be_bytes([*b0, *b1, *b2, *b3]) {
                    Self::DEVICE_STREAM => Ok(Self::DeviceStream),
                    Self::DEVICE_INFO   => Ok(Self::DeviceInfo),
                    Self::DEVICE_DOC    => Ok(Self::DeviceDoc),
                    _ => Err(Error::new("Query", "from_be_bytes").err(format!("Can't parse from bytes {:?}", &bytes[..12]))),
                }
            }
            [] => Err(Error::new("Query", "from_be_bytes").err("Can't parse u32, empty bytes")),
            [..] => Err(Error::new("Query", "from_be_bytes").err(format!("Can't parse u32 from bytes {:?}", bytes))),
        }
    }
}
