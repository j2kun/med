use std::marker::PhantomData;

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
    // The current mode of the document
    current_mode: ModeT,

    // The mode graph for this document
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

    // to make the compiler happe
    phantom: PhantomData<OperationT>,
}

impl<ModeT, ModeGraphT, OperationT, DiffT, DocumentT> Default
    for Editor<ModeT, ModeGraphT, OperationT, DiffT, DocumentT>
where
    OperationT: Operation,
    ModeGraphT: ModeGraph<ModeT, OperationT>,
    DiffT: Diff,
    DocumentT: Document<OperationT, DiffT>,
{
    fn default() -> Self {
        Editor::<ModeT, ModeGraphT, OperationT, DiffT, DocumentT> {
            current_mode: ModeGraphT::initial(),
            mode_graph: ModeGraphT::new(),
            document: DocumentT::new(),
            history_tree: Arena::new(),
            last_edit: None,
            status_line: String::new(),
            phantom: PhantomData,
        }
    }
}

impl<ModeT, ModeGraphT, OperationT, DiffT, DocumentT>
    Editor<ModeT, ModeGraphT, OperationT, DiffT, DocumentT>
where
    OperationT: Operation,
    ModeGraphT: ModeGraph<ModeT, OperationT>,
    DiffT: Diff,
    DocumentT: Document<OperationT, DiffT>,
{
    pub fn process_input(&mut self, chr: char) {
        match self.mode_graph.interpret(chr) {
            TransitionResult::Apply(op, new_mode) => {
                match self.document.interpret(op) {
                    Ok(boxed_diff) => {
                        let diff = *boxed_diff;
                        self.document.apply(&diff);
                        if op.is_edit() {
                            self.add_to_history_tree(diff);
                        }
                    }
                    Err(msg) => {
                        self.status_line = msg.to_string();
                    }
                }
                self.current_mode = new_mode;
            }
            TransitionResult::ModeChange(new_mode, msg) => {
                self.status_line = msg;
                self.current_mode = new_mode;
            }
        }
        self.document.render();
    }

    fn add_to_history_tree(&mut self, diff: DiffT) {
        let new_node_id = self.history_tree.new_node(diff);
        if let Some(node_id) = self.last_edit {
            node_id.append(new_node_id, &mut self.history_tree);
        }
        self.last_edit = Some(new_node_id);
    }
}
