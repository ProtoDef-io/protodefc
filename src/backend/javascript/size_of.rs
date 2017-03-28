use ::ir::{TypeContainer, TypeVariant, TypeData};
use ::ir::variant::Variant;
use super::builder::Block;
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

    match typ_inner.variant {
        Variant::SimpleScalar(ref inner) =>
            JSSizeOf::size_of(inner, &typ_inner.data),
        Variant::Container(ref inner) =>
            JSSizeOf::size_of(inner, &typ_inner.data),
        Variant::Array(ref inner) =>
            JSSizeOf::size_of(inner, &typ_inner.data),
        Variant::SizedBuffer(ref inner) =>
            JSSizeOf::size_of(inner, &typ_inner.data),
        ref variant => {
            println!("Unimplemented variant: {:?}", variant);
            unimplemented!();
        },
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
}

impl JSSizeOf for ::ir::variant::ContainerVariant {
    fn size_of(&self, data: &TypeData) -> Result<Block> {

        let mut b = Block::new();
        b.comment("start container".into());

        for field in &self.fields {
            let mut ib = Block::new();

            let child_typ = field.child.upgrade().unwrap();
            b.comment(format!("container field '{}':", field.name));

            let child_input_var = input_for_type(child_typ.clone());

            use ::ir::variant::ContainerFieldType as CFT;
            match field.field_type {
                CFT::Normal => {
                    b.let_assign(
                        child_input_var.clone(),
                        format!("{}[\"{}\"]", input_for(data), field.name).into()
                    );
                    ib.scope(generate_size_of_inner(child_typ)?);
                }
                CFT::Virtual { ref property } => {
                    // TODO
                    assert!(property.property == "length");

                    let property_target = property.reference_node.clone().unwrap()
                        .upgrade().unwrap();
                    let property_target_inner = property_target.borrow();

                    let property_target_parent = property_target_inner.data.parent
                        .clone().unwrap().upgrade().unwrap();

                    b.let_assign(
                        child_input_var.clone(),
                        format!("{}[\"{}\"].length",
                                input_for_type(property_target_parent),
                                property.reference.name).into()
                    );
                    ib.scope(generate_size_of_inner(child_typ)?);
                }
                _ => unimplemented!(),
            }

            b.block(ib);
        }

        b.comment("end container".into());
        Ok(b)
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
}

impl JSSizeOf for ::ir::variant::SizedBufferVariant {
    fn size_of(&self, _data: &TypeData) -> Result<Block> {
        let mut b = Block::new();

        //let length_input_var = inamer.get();
        //// FIXME: THIS IS WRONG, FIX IT
        //b.let_assign(length_input_var.clone(), format!("{}.length", input_var).into());
        //b.block(generate_size_of_inner(self.length.upgrade().unwrap(),
        //                               input_var, inamer)?);
        //b.assign("count".into(), format!("count + {}", length_input_var).into());

        unimplemented!();
        Ok(b)
    }
}

