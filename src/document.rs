pub trait Diff {
    /**
     * Return the opposite of this diff, such that applying the diff and then applying its inverse
     * is equivalent to doing nothing.
     *
     * Note that this is only a one-sided inverse in general, since the inverse of an operation X
     * may not be well-defined on the document X is applied to, but only well-defined after X is
     * applied.
     */
    fn invert(&self) -> Self;
}

pub trait Operation : Copy {
    // TODO: figure out how to handle undo, what would interpret return???
    // A distinguished undo operation
    fn undo() -> Self;

    // A distinguished redo operation
    fn redo() -> Self;

    /**
     * Return true if this operation is considered an edit, as opposed to an edit-less operation.
     * This determines whether the diff produced by the edit is included in the undo tree.
     */
    fn is_edit(&self) -> bool;
}

pub trait Document<P: Operation, D: Diff> {
    /** Create an empty document. */
    fn new() -> Self;

    /**
     * Interpret an operation on the current document, producing
     * a diff that can then be stored and applied to the document.
     */
    fn interpret(&self, operation: P) -> Result<Box<D>, &str>;

    /**
     * Apply a diff to the current state of the document,
     * mutating it according to the diff.
     */
    fn apply(&mut self, diff: &D);

    // A placeholder for a more complex rendering module
    fn render(&self);
}
