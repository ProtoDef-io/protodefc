use ::errors::*;
use ::ir::spec::{TypeVariant, TypeData, TypeContainer};
use ::ir::spec::variant::*;
use super::*;
use super::utils::*;
use super::container_utils::*;

pub fn generate_size_of(typ: TypeContainer) -> Result<Block> {
    let typ_inner = typ.borrow();
    codegen_for_type(&*typ_inner)
        .size_of(&typ_inner.data)
}

pub trait BaseSizeOf: TypeVariant {
    fn size_of(&self, data: &TypeData) -> Result<Block>;
}

impl BaseSizeOf for SimpleScalarVariant {

    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        ops.push(Operation::TypeCall {
            input: input_for(data).into(),
            output: "size".to_owned().into(),
            typ: CallType::SizeOf,
            type_name: data.name.clone(),
            named_type: self.target.clone().unwrap(),
        });

        ops.push(Operation::AddCount(Expr::Var("size".to_owned().into())));

        Ok(Block(ops))
    }

}

impl BaseSizeOf for ContainerVariant {

    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        // TODO: Do this only allows for one level of virtual field references
        for (idx, field) in self.fields.iter().enumerate() {
            if let ContainerFieldType::Normal = field.field_type {
                build_field_accessor(self, data, &mut ops, idx, false)?;
            }
        }
        for (idx, field) in self.fields.iter().enumerate() {
            if let ContainerFieldType::Virtual { .. } = field.field_type {
                build_field_accessor(self, data, &mut ops, idx, false)?;
            }
        }

        for (_idx, field) in self.fields.iter().enumerate() {
            let child_typ = field.child.upgrade();
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

        let child_input_var = input_for_type(&self.child.upgrade());

        ops.push(Operation::ForEachArray {
            array: input_for(data).into(),
            index: index_var.clone().into(),
            typ: child_input_var.clone().into(),
            block: generate_size_of(self.child.upgrade())?,
        });

        Ok(Block(ops))
    }

}

impl BaseSizeOf for UnionVariant {

    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        let cases: Result<Vec<UnionTagCase>> = self.cases.iter().map(|case| {
            let child_rc = case.child.upgrade();
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
