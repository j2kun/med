pub trait Diff {}

pub trait Operation {}

pub trait Document<P: Operation, D: Diff> {
    /** Create an empty document. */
    fn new() -> Self;

    /** Evaluate a given operation on the current document. */
    fn apply(&self, operation: P) -> Result<Box<D>, &str>;
}
