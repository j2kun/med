pub trait Diff {}

pub trait Operation {
    // A distinguished undo operation
    fn undo() -> Self;

    // A distinguished redo operation
    fn redo() -> Self;
}

pub trait Document<P: Operation, D: Diff> {
    /** Create an empty document. */
    fn new() -> Self;

    /** Evaluate a given operation on the current document. */
    fn apply(&self, operation: P) -> Result<Box<D>, &str>;

    // A placeholder for a more complex rendering module
    fn render(&self) -> String;
}
