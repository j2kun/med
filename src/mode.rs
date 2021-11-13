use crate::document::Operation;

pub enum TransitionResult<M, P: Operation> {
    // An operation emitted, with a next mode to enter.
    Apply(P, M),

    // An action-less mode change, with an optional message.
    ModeChange(M, String),
}

pub trait ModeGraph<M, P: Operation> {
    fn new() -> Self;

    // The initial mode
    fn initial() -> M;

    // Applying an operation can change the mode, or return an error to report to the user, in
    // which case the mode is not changed.
    fn interpret(&self, chr: char) -> TransitionResult<M, P>;
}
