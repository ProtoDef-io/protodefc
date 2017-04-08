use ::errors::*;

pub mod compilation_unit;
pub mod spec;
pub mod type_spec;
pub mod name;

mod field_property_reference;
mod field_reference;
pub use self::field_property_reference::FieldPropertyReference;
pub use self::field_reference::FieldReference;

mod target_type;
pub use self::target_type::TargetType;

mod id_generator;
pub use self::id_generator::IdGenerator;

use ::ir::compilation_unit::{CompilationUnit, TypePath};

use ::rc_container::{Container, WeakContainer};

