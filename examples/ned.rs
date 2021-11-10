/** Ned is an integer Number EDitor using med. Just a silly example.  */
use med::document::Diff;
use med::document::Document;
use med::document::Operation;

// A document is a list of numbers and a selected index.
#[derive(Debug)]
pub struct NedDoc {
    xs: Vec<i64>,
    cursor: usize,
}

#[derive(Debug)]
pub struct NedError {
    message: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AtomicChange {
    ModifyEntry {
        index: usize,
        old_value: i64,
        new_value: i64,
    },
    InsertValue {
        index: usize,
        value: i64,
    },
    DeleteAt {
        index: usize,
        value: i64,
    },
}

#[derive(Debug)]
pub struct NedDiff {
    changes: Vec<AtomicChange>,
    old_cursor: usize,
    new_cursor: usize,
}

#[derive(Debug)]
enum NedOp {
    // Move the cursor left/right
    CursorLeft,
    CursorRight,
    // add/subtract 1 to the number under the cursor
    Increment,
    Decrement,
    // Set current number to zero
    Zero,
    // duplicate the number under the cursor
    Duplicate,
    // add/subtract/multiply the number under the cursor to the next number in the document, merging the two
    // numbers into one entry.
    AddNext,
    SubtractNext,
    MultiplyNext,
    // ditto for the previous number
    AddPrev,
    SubtractPrev,
    MultiplyPrev,
}

use AtomicChange::*;
use NedOp::*;

impl Operation for NedOp {}
impl Diff for NedDiff {}

impl NedDoc {
    fn cursor_only(&self, next_cursor_loc: usize) -> Box<NedDiff> {
        Box::new(NedDiff {
            changes: Vec::new(),
            old_cursor: self.cursor,
            new_cursor: next_cursor_loc,
        })
    }

    fn in_place_change(&self, value: i64) -> Box<NedDiff> {
        Box::new(NedDiff {
            changes: vec![ModifyEntry {
                index: self.cursor,
                old_value: self.xs[self.cursor],
                new_value: value,
            }],
            old_cursor: self.cursor,
            new_cursor: self.cursor,
        })
    }

    fn select_other(&self, other: usize) -> Result<usize, &str> {
        if other > self.xs.len() || other == self.cursor {
            return Err("Cannot apply operation. There are not enough arguments.");
        }
        Ok(other)
    }

    fn merge_with_offset<F>(&self, offset: usize, direction: i8, f: F) -> Result<Box<NedDiff>, &str>
    where
        F: FnOnce(i64, i64) -> i64,
    {
        let other: usize = if direction > 0 {
            self.cursor.wrapping_add(offset)
        } else {
            self.cursor.wrapping_sub(offset)
        };

        self.select_other(other)
            .map(|other| self.merge_with(other, f))
    }

    fn merge_with<F>(&self, other: usize, f: F) -> Box<NedDiff>
    where
        F: FnOnce(i64, i64) -> i64,
    {
        Box::new(NedDiff {
            changes: vec![
                ModifyEntry {
                    index: self.cursor,
                    old_value: self.xs[self.cursor],
                    new_value: f(self.xs[self.cursor], self.xs[other]),
                },
                DeleteAt {
                    index: other,
                    value: self.xs[other],
                },
            ],
            old_cursor: self.cursor,
            new_cursor: self.cursor,
        })
    }
}

impl Document<NedOp, NedDiff> for NedDoc {
    fn new() -> NedDoc {
        NedDoc {
            xs: vec![1],
            cursor: 0,
        }
    }

    fn apply(&self, op: NedOp) -> Result<Box<NedDiff>, &str> {
        match op {
            CursorLeft => Ok(self.cursor_only((self.cursor + 1) % self.xs.len())),
            CursorRight => Ok(self.cursor_only((self.cursor - 1) % self.xs.len())),
            Increment => Ok(self.in_place_change(self.xs[self.cursor] + 1)),
            Decrement => Ok(self.in_place_change(self.xs[self.cursor] - 1)),
            Zero => Ok(self.in_place_change(0)),
            Duplicate => Ok(Box::new(NedDiff {
                changes: vec![InsertValue {
                    index: self.cursor,
                    value: self.xs[self.cursor],
                }],
                old_cursor: self.cursor,
                new_cursor: self.cursor,
            })),
            AddNext => self.merge_with_offset(1, 1, |x, y| x + y),
            SubtractNext => self.merge_with_offset(1, 1, |x, y| x - y),
            MultiplyNext => self.merge_with_offset(1, 1, |x, y| x * y),
            AddPrev => self.merge_with_offset(1, -1, |x, y| x + y),
            SubtractPrev => self.merge_with_offset(1, -1, |x, y| x - y),
            MultiplyPrev => self.merge_with_offset(1, -1, |x, y| x * y),
        }
    }
}

fn main() {}
