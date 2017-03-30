use ::ir::{Type, TypeContainer, WeakTypeContainer, TypeVariant, TypeData};
use ::ir::variant::{Variant, ContainerVariant, ContainerField};
use super::builder::{Block, Expr};
use ::errors::*;

pub fn generate_size_of(typ: TypeContainer) -> Result<Block> {
    let mut ib = Block::new();
    ib.let_assign("count".into(), "0".into());

    {
        let typ_inner = typ.borrow();
        ib.let_assign(input_for(&typ_inner.data), "input".into());
    }

    ib.scope(generate_size_of_inner(typ)?);
    ib.return_("count".into());

    let mut b = Block::new();
    b.decl_fun("".into(), vec!["input".into()], ib);
    Ok(b)
}

fn generate_size_of_inner(typ: TypeContainer) -> Result<Block> {
    let typ_inner = typ.borrow();
    size_of_for_type(&*typ_inner)
        .size_of(&typ_inner.data)
}

fn size_of_for_type<'a>(typ: &'a Type) -> &'a JSSizeOf {
    match typ.variant {
        Variant::SimpleScalar(ref inner) => inner,
        Variant::Container(ref inner) => inner,
        Variant::Array(ref inner) => inner,
        //Variant::Union(ref inner) => inner,
        _ => unimplemented!(),
    }
}

fn input_for(data: &TypeData) -> String {
    format!("type_input_{}", data.ident.unwrap())
}
fn input_for_type(typ: TypeContainer) -> String {
    let typ_inner = typ.borrow();
    input_for(&typ_inner.data)
}

trait JSSizeOf: TypeVariant {
    fn size_of(&self, data: &TypeData) -> Result<Block>;
    fn property_getter(&self, data: &TypeData, name: &str) -> Result<Block>;
}

impl JSSizeOf for ::ir::variant::SimpleScalarVariant {
    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut b = Block::new();
        b.comment(format!("simple scalar ({})", data.name));
        b.assign("count".into(),
                 format!("count + types[\"{}\"][\"size_of\"]({})",
                         data.name, input_for(data)).into());
        Ok(b)
    }
    fn property_getter(&self, data: &TypeData, name: &str) -> Result<Block> {
        unreachable!();
    }
}

use ::ir::variant::ContainerFieldType as CFT;

fn ident_for_weak_type_container(cont: WeakTypeContainer) -> u64 {
    let inner = cont.upgrade().unwrap();
    let inner_b = inner.borrow();
    inner_b.data.ident.unwrap()
}

fn get_field_accessor(variant: &ContainerVariant, data: &TypeData,
                      field: &ContainerField) -> Result<Expr> {
    match field.field_type {
        CFT::Normal => {
            if variant.virt {
                Ok(input_for(data).into())
            } else {
                Ok(format!("{}[\"{}\"]", input_for(data), field.name).into())
            }
        }
        CFT::Virtual { ref property } => {
            assert!(property.property == "length");

            let property_target_ident = ident_for_weak_type_container(
                property.reference_node.clone().unwrap());

            for field in &variant.fields {
                let field_node_ident = ident_for_weak_type_container(
                    field.child.clone());

                if property_target_ident == field_node_ident {
                    
                }
            }
            unimplemented!()
        }
        _ => unimplemented!(),
    }
}

impl JSSizeOf for ::ir::variant::ContainerVariant {
    fn size_of(&self, data: &TypeData) -> Result<Block> {

        let mut b = Block::new();
        b.comment("start container".into());

        for field in &self.fields {
            let child_typ = field.child.upgrade().unwrap();
            let child_input_var = input_for_type(child_typ.clone());
            match field.field_type {
                CFT::Normal => {
                    // Virtual containers are guaranteed to only have 1
                    // non-virtual field.
                    if self.virt {
                        b.let_assign(
                            child_input_var.clone(),
                            input_for(data).into()
                        );
                    } else {
                        b.let_assign(
                            child_input_var.clone(),
                            format!("{}[\"{}\"]", input_for(data), field.name).into()
                        );
                    }
                }
                CFT::Virtual { ref property } => {
                    // TODO
                    assert!(property.property == "length");

                    if self.virt {
                        b.let_assign(
                            child_input_var.clone(),
                            input_for(data).into()
                        );
                    } else {
                        let property_target = property.reference_node
                            .clone().unwrap()
                            .upgrade().unwrap();

                        let property_target_inner = property_target.borrow();

                        let property_target_parent = property_target_inner
                            .data.parent
                            .clone().unwrap().upgrade().unwrap();

                        b.let_assign(
                            child_input_var.clone(),
                            format!("{}[\"{}\"].length",
                                    input_for_type(property_target_parent),
                                    property.reference.name).into()
                        );
                    }
                }
                _ => unimplemented!(),
            }
        }

        for field in &self.fields {
            let mut ib = Block::new();

            let child_typ = field.child.upgrade().unwrap();
            b.comment(format!("container field '{}':", field.name));

            let child_input_var = input_for_type(child_typ.clone());

            match field.field_type {
                CFT::Normal => {
                    ib.scope(generate_size_of_inner(child_typ)?);
                }
                CFT::Virtual { ref property } => {
                    ib.scope(generate_size_of_inner(child_typ)?);
                }
                _ => unimplemented!(),
            }

            b.block(ib);
        }

        b.comment("end container".into());
        Ok(b)
    }
    fn property_getter(&self, data: &TypeData, name: &str) -> Result<Block> {
        unreachable!();
    }
}

impl JSSizeOf for ::ir::variant::ArrayVariant {
    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut b = Block::new();
        b.comment("array".into());

        let mut ib = Block::new();

        let child = self.child.upgrade().unwrap();

        let type_var = input_for(data);
        let count_var = format!("array_current_{}", data.ident.unwrap());
        let length_var = format!("array_length_{}", data.ident.unwrap());
        let child_input_var = input_for_type(child.clone());

        {
            ib.let_assign(child_input_var.clone(),
                         format!("{}[{}]", type_var, count_var).into());
            ib.scope(generate_size_of_inner(child)?);
        }

        b.let_assign(length_var.clone(),
                     format!("{}.length", type_var).into());
        b.for_(
            format!("var {} = 0", count_var).into(),
            format!("{} < {}", count_var, length_var).into(),
            format!("{}++", count_var).into(),
            ib
        );

        Ok(b)
    }
    fn property_getter(&self, data: &TypeData, name: &str) -> Result<Block> {
        match name {
            "length" => {
                let mut b = Block::new();

                let input_var = input_for(data);
                

                Ok(b)
            }
            _ => unreachable!(),
        }
    }
}

