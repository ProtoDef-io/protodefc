use ::ir::TargetType;
use ::ir::spec::{Type, TypeVariant, TypeData, WeakTypeContainer, TypeContainer,
                 CompilePass};
use ::ir::spec::data::SpecReferenceHandle;
use ::ir::spec::variant::{Variant, VariantType};
use ::ir::spec::reference::Reference;
use ::ir::name::Name;
use ::ir::type_spec::WeakTypeSpecContainer;
use ::ir::compilation_unit::{TypePath, NamedTypeContainer, TypeKind, RelativeNSPath,
                             CanonicalNSPath};
use ::errors::*;

/// This is a simple terminal scalar.
///
/// All types that take no special arguments and that have
/// no children should be represented by this variant.
///
/// It is up to the backend to generate code for the name of
/// the type.
#[derive(Debug)]
pub struct SimpleScalarVariant {
    pub relative_path: RelativeNSPath,
    pub path: Option<TypePath>,

    pub target: Option<NamedTypeContainer>,
    pub target_type: Option<TargetType>,

    pub arguments: Vec<SimpleScalarArgument>,
}

#[derive(Debug)]
pub struct SimpleScalarArgument {
    pub name: String,
    pub reference: Reference,
    pub handle: Option<SpecReferenceHandle>,
}

impl TypeVariant for SimpleScalarVariant {
    fn get_type(&self, data: &TypeData) -> VariantType {
        unimplemented!()
        //VariantType::SimpleScalar(data.name.clone())
    }

    default_resolve_child_name_impl!();
    default_has_property_impl!();

    fn do_compile_pass(&mut self, data: &mut TypeData, pass: &mut CompilePass)
                       -> Result<()> {
        match *pass {
            CompilePass::AssignNamespace(ref base_path) => {
                let full_path = base_path.path
                    .concat(&self.relative_path).into_type_path()?;
                self.path = Some(full_path);
                Ok(())
            }
            CompilePass::ResolveReferencedTypes(ref path, ref cu) => {
                let target_resolved = cu.resolve_type(self.path.as_ref().unwrap())?;

                {
                    let target_inner = target_resolved.borrow();
                    for argument in &target_inner.arguments {
                        let reference_arg = self.arguments.iter_mut()
                            .find(|arg| arg.name == argument.name)
                            .ok_or_else(|| format!("required argument '{}' was not supplied",
                                                   argument.name))?;

                        let reference_handle = data.add_reference(
                            reference_arg.reference.clone(), argument.access_time);
                        reference_arg.handle = Some(reference_handle);
                    }
                }

                self.target = Some(target_resolved);
                Ok(())
            }
            CompilePass::MakeTypeSpecs => {
                let named_target_rc = self.target.clone().unwrap();
                let named_target = named_target_rc.borrow();

                data.type_spec = Some(named_target.type_spec.clone());
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl SimpleScalarVariant {

    pub fn new(path: RelativeNSPath,
               references: Vec<(String, Reference)>) -> TypeContainer {
        SimpleScalarVariant::with_target_type(path, references, None)
    }

    pub fn with_target_type(path: RelativeNSPath,
                            mut references: Vec<(String, Reference)>,
                            target_type: Option<TargetType>) -> TypeContainer {
        let mut data = TypeData::default();

        let arguments = references.drain(..)
            .map(|(string, reference)| SimpleScalarArgument {
                name: string,
                reference: reference,
                handle: None,
            })
            .collect();

        TypeContainer::new(Type {
            data: data,
            variant: Variant::SimpleScalar(SimpleScalarVariant {
                relative_path: path,
                path: None,
                target: None,
                target_type: target_type,
                arguments: arguments,
            }),
        })
    }

}
