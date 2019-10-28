//!
//! Contains `Chain` trait and implementation, expressions, groups and utils to work with all of them.
//!

pub mod action_expr_chain;
pub mod chain;
pub mod expr;
pub mod group;
pub mod utils;

pub use action_expr_chain::ActionExprChain;
pub use chain::Chain;
use chain::{Unit, UnitResult};
pub use expr::ActionExpr;
use group::{ActionGroup, CommandGroup, GroupDeterminer};
use utils::{is_block_expr, is_valid_expr, parse_until};
