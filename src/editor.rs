use indextree::Arena;
use indextree::NodeId;

use crate::document::Diff;
use crate::document::Document;
use crate::document::Operation;

#[derive(Debug)]
pub struct Editor<M, O, F, D, R>
where
    M: Mode,
    O: Operation,
    F: Diff,
    D: Document<O, F>,
    R: Renderer,
{
    current_mode: Mode,

    // The document, as materialized with respect to the current
    // position in the history tree.
    document: Graph,

    // The "undo tree" of this editing session
    // Children of a given node are appended in time order.
    history_tree: Arena<Diff>,

    // The id of the node in the history tree corresponding
    // to the last edit of the document.
    last_edit: Option<NodeId>,

    status_line: &str,
}

impl Editor<M, O, F, D> {
    pub fn new() -> Self {
        // TODO
        // pass to Document::new
        // user passes in the mode state machine
        // store the default starting mode
        // define which inputs map to undo/redo in which modes
    }

    pub fn process_input(&mut self, chr: char) {
       // TODO
       // interpret input with mode
       // if result emits a new operation:
           // interpret operation on document
           // apply resulting diff to self.document
       // Stop if any errors and write them to the status line
       // render
    }
}
