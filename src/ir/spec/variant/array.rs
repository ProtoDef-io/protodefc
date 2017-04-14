use ::errors::*;
use ::ir::TargetType;
use ::ir::spec::{TypeVariant, TypeData, Type, WeakTypeContainer, TypeContainer, CompilePass};
use ::ir::spec::data::{SpecChildHandle, SpecReferenceHandle};
use ::ir::spec::reference::Reference;
use ::ir::type_spec::{TypeSpecContainer, WeakTypeSpecContainer, TypeSpecVariant,
                      ArraySpec, ArraySize, IntegerSpec, Signedness, IntegerSize};
use ::ir::compilation_unit::{CompilationUnit, TypePath};
use super::{Variant, VariantType};

use std::rc::Rc;
use std::cell::RefCell;

// Array
#[derive(Debug)]
pub struct ArrayVariant {
    pub count_reference: Reference,
    pub count_handle: SpecReferenceHandle,

    pub child: WeakTypeContainer,
    pub child_handle: SpecChildHandle,

    pub count_type: Option<TypeSpecContainer>,
}
impl TypeVariant for ArrayVariant {
    default_resolve_child_name_impl!();

    fn get_type(&self, _data: &TypeData) -> VariantType {
        VariantType::Array
    }

    fn has_spec_property(&self, _data: &TypeData, name: &str)
                    -> Result<Option<WeakTypeSpecContainer>> {
        match name {
            "length" => Ok(Some(self.count_type.clone().unwrap().downgrade())),
            _ => bail!("array variant has no property '{}'"),
        }
    }

    fn do_compile_pass(&mut self, data: &mut TypeData, pass: &mut CompilePass)
                       -> Result<()> {
        match *pass {
            CompilePass::MakeTypeSpecs => {
                let child_rc = self.child.clone().upgrade();
                let child = child_rc.borrow();
                data.type_spec = Some(TypeSpecVariant::Array(ArraySpec {
                    size: ArraySize::Dynamic,
                    child: child.data.type_spec.clone().unwrap(),
                }).into());
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl ArrayVariant {

    pub fn new(count_ref: Reference, child: TypeContainer) -> TypeContainer {
        let mut data = TypeData::default();
        data.name = TypePath::with_no_ns("array".to_owned());

        let child_handle = data.add_child(child.clone());
        let count_reference_handle = data.add_reference(count_ref.clone());

        TypeContainer::new(Type {
            variant: Variant::Array(ArrayVariant {
                count_reference: count_ref,
                count_handle: count_reference_handle,

                child: child.downgrade(),
                child_handle: child_handle,

                // TODO
                count_type: Some(TypeSpecVariant::Integer(IntegerSpec {
                    signed: Signedness::Signed,
                    size: IntegerSize::B64,
                }).into()),
            }),
            data: data,
        })
    }

}
