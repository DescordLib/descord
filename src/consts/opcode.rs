#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatACK = 11,
}

impl OpCode {
    pub fn parse(code: u8) -> Option<OpCode> {
        Some(match code {
            0 => OpCode::Dispatch,
            1 => OpCode::Heartbeat,
            2 => OpCode::Identify,
            3 => OpCode::PresenceUpdate,
            4 => OpCode::VoiceStateUpdate,
            6 => OpCode::Resume,
            7 => OpCode::Reconnect,
            8 => OpCode::RequestGuildMembers,
            9 => OpCode::InvalidSession,
            10 => OpCode::Hello,
            11 => OpCode::HeartbeatACK,

            _ => return None,
        })
    }
}
