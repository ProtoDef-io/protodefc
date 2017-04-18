use ::errors::*;

use ::ir::spec::{TypeVariant, TypeData, Type, WeakTypeContainer, TypeContainer, CompilePass};
use ::ir::spec::variant::{Variant, VariantType};
use ::ir::spec::data::{SpecChildHandle, SpecReferenceHandle, ReferenceAccessTime};
use ::ir::spec::reference::Reference;
use ::ir::type_spec::{WeakTypeSpecContainer, TypeSpecVariant,
                      EnumSpec, EnumVariantSpec};

#[derive(Debug)]
pub struct UnionVariant {
    pub union_name: String,

    pub match_target_handle: SpecReferenceHandle,

    pub cases: Vec<UnionCase>,
    pub default_case: Option<UnionCase>,

    pub tag_property_type: Option<WeakTypeSpecContainer>,
}

#[derive(Debug)]
pub struct UnionCase {
    pub match_val_str: String,
    pub case_name: String,

    pub child: WeakTypeContainer,
    pub child_handle: SpecChildHandle,
}

impl TypeVariant for UnionVariant {
    default_resolve_child_name_impl!();

    fn has_spec_property(&self, data: &TypeData, name: &str)
                         -> Result<Option<WeakTypeSpecContainer>> {
        // TODO: Infer type
        match name {
            //"tag" => Ok(self.tag_property_type.clone()),
            "tag" => Ok(data.get_reference_data(self.match_target_handle).target_type_spec.clone().map(|i| i.downgrade())),
            _ => bail!("union variant has no property '{}'", name),
        }
    }

    fn get_type(&self, _data: &TypeData) -> VariantType {
        VariantType::Union
    }

    fn do_compile_pass(&mut self, data: &mut TypeData, pass: &mut CompilePass)
                       -> Result<()> {
        match *pass {
            CompilePass::MakeTypeSpecs => {
                data.type_spec = Some(TypeSpecVariant::Enum(EnumSpec {
                    name: self.union_name.clone().into(),
                    variants: self.cases.iter().map(|c| {
                        let child_rc = c.child.clone().upgrade();
                        let child = child_rc.borrow();
                        EnumVariantSpec {
                            name: c.case_name.clone().into(),
                            type_spec: child.data.type_spec.clone().unwrap(),
                        }
                    }).collect(),
                }).into());
                Ok(())
            }
            _ => Ok(())
        }
    }
}

pub struct UnionVariantBuilder {
    typ: Type,
}
impl UnionVariantBuilder {

    pub fn new(union_name: String, match_target: Reference)
               -> UnionVariantBuilder {
        let mut data = TypeData::default();
        let match_target_handle = data.add_reference(match_target.clone(),
                                                     ReferenceAccessTime::Read);

        UnionVariantBuilder {
            typ: Type {
                data: data,
                variant: Variant::Union(UnionVariant {
                    union_name: union_name,

                    match_target_handle: match_target_handle,

                    cases: Vec::new(),
                    default_case: None,

                    tag_property_type: None,
                })
            }
        }
    }

    pub fn case(&mut self, match_val_str: String, case_name: String,
                child: TypeContainer) {
        let case_handle = self.typ.data.add_child(child.clone());
        match self.typ.variant {
            Variant::Union(ref mut variant) => {
                variant.cases.push(UnionCase {
                    match_val_str: match_val_str,
                    case_name: case_name,
                    child: child.downgrade(),
                    child_handle: case_handle,
                });
            }
            _ => unreachable!(),
        }
    }

    pub fn default(&mut self, case_name: String, child: TypeContainer) -> Result<()> {
        match self.typ.variant {
            Variant::Union(ref mut variant) => {
                ensure!(variant.default_case.is_none(),
                        "duplicate default variant");
                let case_handle = self.typ.data.add_child(child.clone());
                variant.default_case = Some(UnionCase {
                    match_val_str: "".to_owned(), // TODO
                    case_name: case_name,
                    child: child.downgrade(),
                    child_handle: case_handle,
                });
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    pub fn build(self) -> ::std::result::Result<TypeContainer, String> {
        Ok(TypeContainer::new(self.typ))
    }

}
