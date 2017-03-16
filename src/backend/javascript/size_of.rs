use ::{TypeContainer, TypeVariant, TypeData};
use ::variants::Variant;
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

fn generate_size_of(typ: TypeContainer) -> Result<Block> {
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
        Variant::PrefixedString(ref inner) =>
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

impl JSSizeOf for ::variants::SimpleScalarVariant {
    fn size_of(&self, data: &TypeData, input_var: &str, inamer: &mut InputNamer) -> Result<Block> {
        let mut b = Block::new();
        b.assign("count".into(),
                 format!("count + types[\"{}\"][\"size_of\"]({})",
                         data.name, input_var).into());
        Ok(b)
    }
}

impl JSSizeOf for ::variants::ContainerVariant {
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

impl JSSizeOf for ::variants::FixedArrayVariant {
    fn size_of(&self, _data: &TypeData, input_var: &str, inamer: &mut InputNamer) -> Result<Block> {
        let len_input_var = inamer.get();

        let mut b = Block::new();
        //b.block(generate_size_of_inner(self.child.upgrade().unwrap())?);
        b.assign("count".into(),
                 format!("count + (int_count * {})", self.count).into());
        //Ok(b);
        unimplemented!();
    }
}

impl JSSizeOf for ::variants::PrefixedStringVariant {
    fn size_of(&self, _data: &TypeData, input_var: &str, inamer: &mut InputNamer) -> Result<Block> {
        let mut b = Block::new();

        let length_input_var = inamer.get();
        // FIXME: THIS IS WRONG, FIX IT
        b.let_assign(length_input_var.clone(), format!("{}.length", input_var).into());
        b.block(generate_size_of_inner(self.length.upgrade().unwrap(),
                                       input_var, inamer)?);
        b.assign("count".into(), format!("count + {}", length_input_var).into());

        Ok(b)
    }
}

#[cfg(all(test, feature = "js_tests"))]
mod tests {
    use ::json_to_final_ast;
    use super::generate_size_of;
    use super::super::builder::ToJavascript;

    #[test]
    fn simple_scalar() {
        let ast = json_to_final_ast("[\"i8\", null]").unwrap();
        let size_of = generate_size_of(ast).unwrap();

        let mut out = String::new();
        size_of.to_javascript(&mut out, 0)
    }

    #[test]
    fn container() {
        let ast = json_to_final_ast(r#"
["container", [
    {"name": "foo", "type": "i8"},
    {"name": "bar", "type": "i8"}
]]"#).unwrap();
        let size_of = generate_size_of(ast).unwrap();

        let mut out = String::new();
        size_of.to_javascript(&mut out, 0);

        println!("{}", out);
        super::super::test_harness::test_with_data_eq(&out, "{foo: 0, bar: 0}", "2");
    }

    #[test]
    fn protodef_spec_tests() {
        for case in ::test_harness::cases() {
            println!("Testing {}", case.name);

            let ast = ::json_to_final_ast(&::json::stringify(case.json_type)).unwrap();
            let size_of = generate_size_of(ast).unwrap();

            let mut out = String::new();
            size_of.to_javascript(&mut out, 0);

            for value in case.values {
                super::super::test_harness::test_with_data_eq(
                    &out,
                    &value.json_value,
                    &format!("{}", value.serialized.len()));
            }
        }
    }

}

