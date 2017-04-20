use ::errors::*;
use ::ir::spec::{TypeVariant, TypeData, Type, WeakTypeContainer, TypeContainer, CompilePass};
use ::ir::spec::data::{SpecChildHandle, SpecReferenceHandle, ReferenceAccessTime};
use ::ir::spec::reference::Reference;
use ::ir::name::Name;
use ::ir::type_spec::{TypeSpecContainer, WeakTypeSpecContainer, TypeSpecVariant,
                      ArraySpec, ArraySize, IntegerSpec, IntegerSize};
use ::ir::compilation_unit::TypePath;
use super::{Variant, VariantType};

// Array
#[derive(Debug)]
pub struct ArrayVariant {
    pub count_reference: Reference,
    pub count_handle: SpecReferenceHandle,

    pub child: WeakTypeContainer,
    pub child_handle: SpecChildHandle,
}
impl TypeVariant for ArrayVariant {
    default_resolve_child_name_impl!();

    fn get_type(&self, _data: &TypeData) -> VariantType {
        VariantType::Array
    }

    fn has_spec_property(&self, _data: &TypeData, _name: &Name)
                    -> Result<Option<WeakTypeSpecContainer>> {
        bail!("array variant has no property {:?}")
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
            CompilePass::ValidateTypes => {
                let type_spec_rc = data.get_reference_data(self.count_handle)
                    .target_type_spec.clone().unwrap().follow();
                let type_spec = type_spec_rc.borrow();
                println!("{:?}", type_spec);

                ensure!(type_spec.variant.is_integer(),
                        "array length property must be integer");
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
        let count_reference_handle = data.add_reference(
            count_ref.clone(), ReferenceAccessTime::Read);

        TypeContainer::new(Type {
            variant: Variant::Array(ArrayVariant {
                count_reference: count_ref,
                count_handle: count_reference_handle,

                child: child.downgrade(),
                child_handle: child_handle,
            }),
            data: data,
        })
    }

}
