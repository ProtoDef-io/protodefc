use ::errors::*;
use ::ir::spec::{TypeVariant, TypeData, TypeContainer};
use ::ir::spec::variant::*;
use super::*;
use super::utils::*;
use super::reference::build_reference_accessor;

pub fn generate_deserialize(typ: TypeContainer) -> Result<Block> {
    let typ_inner = typ.borrow();
    codegen_for_type(&*typ_inner)
        .deserialize(&typ_inner.data)
}

pub trait BaseDeserialize: TypeVariant {
    fn deserialize(&self, data: &TypeData) -> Result<Block>;
}

impl BaseDeserialize for SimpleScalarVariant {
    fn deserialize(&self, data: &TypeData) -> Result<Block> {
        Ok(Block(vec![
            Operation::TypeCall {
                typ: CallType::Deserialize,
                type_name: data.name.clone().into(),
                input: "buffer".to_owned().into(),
                output: output_for(data).into(),
                named_type: self.target.clone().unwrap(),
            }
        ]))
    }
}

fn field_is_normal(field: &ContainerField) -> bool {
    match field.field_type {
        ContainerFieldType::Normal => true,
        _ => false,
    }
}

impl BaseDeserialize for ContainerVariant {
    fn deserialize(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        for (_idx, field) in self.fields.iter().enumerate() {
            let child_typ = field.child.upgrade();

            ops.push(Operation::Block(generate_deserialize(child_typ)?));
        }

        if self.virt {
            let real_field = self.fields.iter()
                .find(|f| field_is_normal(f))
                .unwrap();
            let real_field_out = output_for_type(&real_field.child.upgrade());

            ops.push(Operation::Assign {
                name: output_for(data).into(),
                value: Expr::Var(real_field_out.into()),
            })
        } else {
            ops.push(Operation::ConstructContainer {
                output: output_for(data).into(),
                fields: self.fields.iter()
                    .filter(|f| field_is_normal(f))
                    .map(|field| (
                        field.name.clone(),
                        output_for_type(&field.child.upgrade()).into()
                    ))
                    .collect(),
            });
        }

        Ok(Block(ops))
    }
}

impl BaseDeserialize for ArrayVariant {
    fn deserialize(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        let count_var = var_for("count", data);
        let count_root_node = data.get_reference_root(self.count_handle).upgrade();
        ops.push(Operation::Block(build_reference_accessor(
            self, data, self.count_handle, count_var.clone().into(), true)));

        let child_rc = self.child.upgrade();
        let child_var = output_for_type(&child_rc);

        let ident = data.ident.unwrap();
        let item_var = format!("array_{}_index", ident);

        ops.push(Operation::ConstructArray {
            count: count_var.into(),
            ident: ident,
            item_var: child_var.into(),
            block: generate_deserialize(child_rc)?,
            output: output_for(data).into(),
        });

        Ok(Block(ops))
    }
}

impl BaseDeserialize for UnionVariant {
    fn deserialize(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        let tag_root_node = data.get_reference_root(self.match_target_handle).upgrade();
        let tag_var = var_for_type("tag", &tag_root_node);
        ops.push(Operation::Block(build_reference_accessor(
            self, data, self.match_target_handle, tag_var.clone().into(), true)));

        let out_var = output_for(data);

        let cases: Result<Vec<LiteralCase>> = self.cases.iter()
            .map(|case| {
                let child_rc = case.child.upgrade();
                let mut i_ops: Vec<Operation> = Vec::new();

                generate_deserialize(child_rc.clone()).map(|v| {
                    i_ops.push(Operation::Block(v));
                    i_ops.push(Operation::ConstructUnion {
                        union_name: self.union_name.clone(),
                        union_tag: case.case_name.clone(),
                        output: out_var.clone().into(),
                        input: output_for_type(&child_rc).into(),
                    });

                    LiteralCase {
                        value: Literal::Number(case.match_val_str.clone()),
                        block: Block(i_ops),
                    }
                })
            })
            .collect();

        ops.push(Operation::MapValue {
            input: tag_var.into(),
            output: "".to_owned().into(),
            operation: MapOperation::LiteralToExpr(cases?),
        });

        Ok(Block(ops))
    }
}
