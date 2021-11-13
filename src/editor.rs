use indextree::Arena;
use indextree::NodeId;

use crate::document::Diff;
use crate::document::Document;
use crate::document::Operation;
use crate::mode::ModeGraph;
use crate::mode::TransitionResult;

pub struct Editor<ModeT, ModeGraphT, OperationT, DiffT, DocumentT>
where
    OperationT: Operation,
    ModeGraphT: ModeGraph<ModeT, OperationT>,
    DiffT: Diff,
    DocumentT: Document<OperationT, DiffT>,
{
    current_mode: ModeT,
    mode_graph: ModeGraphT,

    // The document, as materialized with respect to the current
    // position in the history tree.
    document: DocumentT,

    // The "undo tree" of this editing session
    // Children of a given node are appended in time order.
    history_tree: Arena<DiffT>,

    // The id of the node in the history tree corresponding
    // to the last edit of the document.
    last_edit: Option<NodeId>,

    status_line: String,
}

impl<ModeT, ModeGraphT, OperationT, DiffT, DocumentT>
    Editor<ModeT, ModeGraphT, OperationT, DiffT, DocumentT>
where
    OperationT: Operation,
    ModeGraphT: ModeGraph<ModeT, OperationT>,
    DiffT: Diff,
    DocumentT: Document<OperationT, DiffT>,
{
    pub fn new() -> Self {
        Editor::<ModeT, ModeGraphT, OperationT, DiffT, DocumentT> {
            current_mode: ModeGraphT::initial(),
            mode_graph: ModeGraphT::new(),
            document: DocumentT::new(),
            history_tree: Arena::new(),
            last_edit: None,
            status_line: String::new(),
        }
    }

    pub fn process_input(&mut self, chr: char) {
        // TODO
        
        // if result emits a new operation:
        // interpret operation on document
        // apply resulting diff to self.document
        // Stop if any errors and write them to the status line
        // render
    }
}
