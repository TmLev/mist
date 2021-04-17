pub mod vdb12;

use crate::vdb12::Context;

pub enum Algorithm {
    Vdb12,
}

pub enum AlgorithmContext {
    Vdb12(Context),
}
