use ::{TypeVariant, TypeData, Type, WeakTypeContainer, Result, TypeContainer, CompilerError};
use ::ir::TargetType;
use ::ir::variant::{Variant, VariantType};
use ::FieldReference;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct UnionVariant {
    pub union_name: String,

    pub match_field_ref: FieldReference,
    pub match_field: Option<WeakTypeContainer>,
    pub match_type: Option<TargetType>,

    pub cases: Vec<UnionCase>,
}

#[derive(Debug)]
pub struct UnionCase {
    pub match_val_str: String,
    pub case_name: String,
    pub child: WeakTypeContainer,
}

impl TypeVariant for UnionVariant {
    default_resolve_child_name_impl!();

    fn has_property(&self, _data: &TypeData, name: &str) -> Option<TargetType> {
        // TODO: Infer type
        match name {
            "tag" => Some(TargetType::Integer),
            _ => None,
        }
    }

    fn get_type(&self, _data: &TypeData) -> VariantType {
        VariantType::Union
    }

    fn get_result_type(&self, _data: &TypeData) -> Option<TargetType> {
        Some(TargetType::Enum)
    }

    fn do_resolve_references(&mut self, data: &mut TypeData,
                             resolver: &::ReferenceResolver) -> Result<()> {

        self.match_field = Some(resolver(self, data, &self.match_field_ref)?);

        let match_field = self.match_field.clone().unwrap().upgrade().unwrap();
        let match_field_inner = match_field.borrow();
        let match_field_type = match_field_inner.variant.to_variant()
            .get_result_type(&match_field_inner.data);

        ensure!(match_field_type != None, CompilerError::UnmatchableType {
            variant: match_field_inner.variant.get_type(&match_field_inner.data),
        });

        ensure!(match_field_type != None,
                "attempted to match on a unmatchable type");
        self.match_type = match_field_type;

        Ok(())
    }
}

pub struct UnionVariantBuilder {
    typ: Type,
}
impl UnionVariantBuilder {

    pub fn new(union_name: String, match_field: FieldReference)
               -> UnionVariantBuilder {
        UnionVariantBuilder {
            typ: Type {
                data: TypeData::default(),
                variant: Variant::Union(UnionVariant {
                    union_name: union_name,

                    match_field_ref: match_field,
                    match_field: None,
                    match_type: None,

                    cases: Vec::new(),
                })
            }
        }
    }

    pub fn case(&mut self, match_val_str: String, case_name: String,
                child: TypeContainer) {
        match self.typ.variant {
            Variant::Union(ref mut variant) => {
                variant.cases.push(UnionCase {
                    match_val_str: match_val_str,
                    case_name: case_name,
                    child: Rc::downgrade(&child),
                });
            }
            _ => unreachable!(),
        }
        self.typ.data.children.push(child);
    }

    pub fn build(self) -> ::std::result::Result<TypeContainer, String> {
        Ok(Rc::new(RefCell::new(self.typ)))
    }

}
