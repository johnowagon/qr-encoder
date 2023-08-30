// Determining modes can be thought of as a state machine

use crate::modes::Mode;

// I feel like theres a better way of doing this, but for now its okay.
// Just results in some duplicated code between structs, so I was thinking
// that a custom #[derive] could acheive the same result.
trait Checker {
    fn check_tokens<St: GetTokens>(self, s: &St);
}

trait GetTokens {
    fn get_tokens(self) -> String;
} // What our structs that specify tokens will implement
                   // so we can use a generic to access them

struct ModeStateMachine<S> {
    state: S,
    data: String
}

impl<S> ModeStateMachine<S> {
    fn next(self) {
        // Moves state to the next state.
        match self.state {
            Numeric => {
                self.state = AlphaNumeric::from(self.state);
            },

        }
    }
}

impl<S> Checker for ModeStateMachine<S> {
    fn check_tokens<St: GetTokens>(self, s: &St) {
        for c in self.data.chars() {
            if !s.get_tokens().contains(c) {
                self.next();
            }
        }
    }
}

// Starting point for the state machine will be the numeric state as it is the
// most strict in its requirements.
impl ModeStateMachine<Numeric> {
    fn new(data: String) -> Self {
        ModeStateMachine { 
            state: Numeric{ 
                tokens: String::from("0123456789") 
            }, 
            data: data
        }
    }
}

// Numeric state
struct Numeric {
    tokens: String
}
impl GetTokens for Numeric {
    fn get_tokens(self) -> String {
        return self.tokens;
    }
}

// Next state is the AlphaNumeric state.
impl From<Numeric> for AlphaNumeric {
    fn from(m: Numeric) -> AlphaNumeric {
        AlphaNumeric { 
            tokens: String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$%*+-./: ") 
        }
    }
}

struct AlphaNumeric {
    tokens: String
}
impl GetTokens for AlphaNumeric {
    fn get_tokens(self) -> String {
        return self.tokens;
    }
}

// Next state is the Kanji state.
impl From<ModeStateMachine<AlphaNumeric>> for ModeStateMachine<Kanji> {
    fn from(m: ModeStateMachine<AlphaNumeric>) -> ModeStateMachine<Kanji> {
        ModeStateMachine { 
            state: Kanji { tokens: String::from("亜") }, // Place a global list of these elsewhere, future work.
            data: m.data 
        }
    }
}

// Kanji type
struct Kanji {
    tokens: String
}
impl GetTokens for Kanji {
    fn get_tokens(self) -> String {
        return self.tokens;
    }
}

impl From<ModeStateMachine<Kanji>> for ModeStateMachine<Byte> {
    fn from(m: ModeStateMachine<Kanji>) -> ModeStateMachine<Byte> {
        ModeStateMachine { 
            state: Byte, // Place a global list of these elsewhere, future work.
            data: m.data 
        }
    }
}

// Unit struct as a catch all for generic types of data.
struct Byte;