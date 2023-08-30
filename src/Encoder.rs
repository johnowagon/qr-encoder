// A encoding algorithm can be thought of as a Finite State Machine (FSM)
// with the data being encoded as the state. This is an implementation of
// a FSM that encodes given data into the QR encoding.

use crate::{modes::Mode, codes::ErrorCorrectionLevel};

struct Encoder<S> {
    // Our state machine holds a generic state, and we implement the ways 
    // for it to change its state.
    state: S
}

// Our state machine starts in the 'raw' state.
impl Encoder<Raw> {
    fn new(data: String) -> Self {
        Encoder {
            state: Raw::new(data)
        }
    }
}

// Raw data of QR code.
struct Raw {
    data: String
}
impl Raw {
    fn new(data: String) -> Self {
        Raw {
            data: data
        }
    }
}

// We need to determine some characteristics of the data before we can 
// encode it, so we define a transition from Raw to Specified.
struct Specified {
    mode: Mode,
    correction_level: ErrorCorrectionLevel,
    version: u16,
    data: String // We need to keep the data
}
impl From<Encoder<Raw>> for Encoder<Specified> {
    fn from(e: Encoder<Raw>) -> Encoder<Specified> {
        Encoder {
            state: Specified { 
                mode: (), 
                correction_level: (), 
                version: (), 
                data: e.state.data 
            }
        }
    }
}