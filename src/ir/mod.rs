use ::errors::*;

pub mod compilation_unit;
pub mod spec;
pub mod type_spec;
pub mod name;

mod id_generator;
pub use self::id_generator::IdGenerator;

use ::ir::compilation_unit::{CompilationUnit, TypePath};

use ::rc_container::{Container, WeakContainer};

mod target_type;
pub use self::target_type::TargetType;
