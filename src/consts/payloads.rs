use json::object;
use json::JsonValue;

pub fn heartbeat(last_sequence: usize) -> JsonValue {
    if last_sequence == 0 {
        object! {
            op: 1,
            d: null
        }
    } else {
        object! {
            op: 1,
            d: last_sequence
        }
    }
}

pub fn identify(token: &str, intents: u32) -> JsonValue {
    object! {
        op: 2,
        d: {
            token: token,
            properties: {
                os: "linux",
                browser: "descord",
                device: "descord"
            },
            intents: intents
        }
    }
}

pub fn resume(token: &str, session_id: &str, seq: usize) -> JsonValue {
    object! {
        op: 6,
        d: {
            token: token,
            seq: seq,
            session_id: session_id
        }
    }
}
