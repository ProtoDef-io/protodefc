use ::ir::{TypeContainer, TypeVariant, TypeData};
use ::ir::variant::Variant;
use super::builder::Block;
use ::errors::*;

struct InputNamer(usize);
impl InputNamer {
    fn new() -> InputNamer { InputNamer(0) }
    fn get(&mut self) -> String {
        self.0 += 1;
        format!("input_{}", self.0)
    }
}

pub fn generate_size_of(typ: TypeContainer) -> Result<Block> {
    let mut inamer = InputNamer::new();

    let mut ib = Block::new();
    ib.let_assign("count".into(), "0".into());
    ib.block(generate_size_of_inner(typ, "input", &mut inamer)?);
    ib.return_("count".into());

    let mut b = Block::new();
    b.decl_fun("".into(), vec!["input".into()], ib);
    Ok(b)
}

fn generate_size_of_inner(typ: TypeContainer, input_var: &str, inamer: &mut InputNamer) -> Result<Block> {
    let typ_inner = typ.borrow();

    match typ_inner.variant {
        Variant::SimpleScalar(ref inner) =>
            JSSizeOf::size_of(inner, &typ_inner.data, input_var, inamer),
        Variant::Container(ref inner) =>
            JSSizeOf::size_of(inner, &typ_inner.data, input_var, inamer),
        Variant::SizedBuffer(ref inner) =>
            JSSizeOf::size_of(inner, &typ_inner.data, input_var, inamer),
        ref variant => {
            println!("Unimplemented variant: {:?}", variant);
            unimplemented!();
        },
    }
}

trait JSSizeOf: TypeVariant {
    fn size_of(&self, data: &TypeData, input_var: &str, inamer: &mut InputNamer) -> Result<Block>;
}

impl JSSizeOf for ::ir::variant::SimpleScalarVariant {
    fn size_of(&self, data: &TypeData, input_var: &str, inamer: &mut InputNamer) -> Result<Block> {
        let mut b = Block::new();
        b.assign("count".into(),
                 format!("count + types[\"{}\"][\"size_of\"]({})",
                         data.name, input_var).into());
        Ok(b)
    }
}

impl JSSizeOf for ::ir::variant::ContainerVariant {
    fn size_of(&self, _data: &TypeData, input_var: &str, inamer: &mut InputNamer) -> Result<Block> {

        let mut b = Block::new();

        for field in &self.fields {
            let mut ib = Block::new();

            let cont_input_var = inamer.get();
            b.let_assign(cont_input_var.clone(),
                         format!("{}[\"{}\"]", input_var, field.name).into());
            ib.block(generate_size_of_inner(field.child.upgrade().unwrap(),
                                            &cont_input_var, inamer)?);

            b.block(ib);
        }
        Ok(b)
    }
}

impl JSSizeOf for ::ir::variant::ArrayVariant {
    fn size_of(&self, _data: &TypeData, input_var: &str, inamer: &mut InputNamer) -> Result<Block> {
        let len_input_var = inamer.get();

        let mut b = Block::new();
        //b.block(generate_size_of_inner(self.child.upgrade().unwrap())?);
        //b.assign("count".into(),
        //         format!("count + (int_count * {})", self.count).into());
        //Ok(b);
        unimplemented!();
    }
}

impl JSSizeOf for ::ir::variant::SizedBufferVariant {
    fn size_of(&self, _data: &TypeData, input_var: &str, inamer: &mut InputNamer) -> Result<Block> {
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

