use ::errors::*;
use ::ir::{Type, TypeVariant, TypeData, TypeContainer};
use ::ir::variant::{Variant, SimpleScalarVariant, ContainerVariant, ArrayVariant, UnionVariant};
use super::*;
use super::utils::*;
use super::container_utils::*;

pub fn generate_size_of(typ: TypeContainer) -> Result<Block> {
    let typ_inner = typ.borrow();
    size_of_for_type(&*typ_inner)
        .size_of(&typ_inner.data)
}

fn size_of_for_type<'a>(typ: &'a Type) -> &'a BaseSizeOf {
    match typ.variant {
        Variant::SimpleScalar(ref inner) => inner,
        Variant::Container(ref inner) => inner,
        Variant::Array(ref inner) => inner,
        Variant::Union(ref inner) => inner,
        _ => unimplemented!(),
    }
}

trait BaseSizeOf: TypeVariant {
    fn size_of(&self, data: &TypeData) -> Result<Block>;
}

impl BaseSizeOf for SimpleScalarVariant {

    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        ops.push(Operation::AddCount(Expr::TypeCall {
            typ: CallType::SizeOf,
            type_name: data.name.clone().into(),
            input: input_for(data).into(),
        }));

        Ok(Block(ops))
    }

}

impl BaseSizeOf for ContainerVariant {

    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        for (idx, field) in self.fields.iter().enumerate() {
            let child_typ = field.child.upgrade().unwrap();

            build_var_accessor(self, data, &mut ops, idx)?;
            ops.push(Operation::Block(generate_size_of(child_typ)?));
        }

        Ok(Block(ops))
    }

}

impl BaseSizeOf for ArrayVariant {

    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        let ident = data.ident.unwrap();
        let index_var = format!("array_{}_index", ident);

        let child_input_var = input_for_type(self.child.upgrade().unwrap());

        ops.push(Operation::ForEachArray {
            array: input_for(data).into(),
            index: index_var.clone().into(),
            typ: child_input_var.clone().into(),
            block: generate_size_of(self.child.upgrade().unwrap())?,
        });

        Ok(Block(ops))
    }

}

impl BaseSizeOf for UnionVariant {

    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        let cases: Result<Vec<UnionTagCase>> = self.cases.iter().map(|case| {
            let child_rc = case.child.upgrade().unwrap();
            let child_inner = child_rc.borrow();

            let mut i_ops: Vec<Operation> = Vec::new();

            let inner = generate_size_of(child_rc.clone())?;
            i_ops.push(Operation::Block(inner));

            Ok(UnionTagCase {
                variant_name: case.case_name.clone(),
                variant_var: Some(
                    input_for(&child_inner.data).into()),
                block: Block(i_ops),
            })
        }).collect();

        ops.push(Operation::MapValue {
            input: input_for(data).into(),
            output: "".to_owned().into(),
            operation: MapOperation::UnionTagToExpr(cases?),
        });

        Ok(Block(ops))
    }

}
