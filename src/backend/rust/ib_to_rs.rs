use ::backend::imperative_base as ib;
use ::errors::*;
use super::builder::*;

pub fn build_block(block: &ib::Block) -> Result<Block> {
    let mut b = Block::new();

    for operation in &block.0 {
        build_operation(operation, &mut b)?;
    }

    Ok(b)
}

pub fn build_operation(operation: &ib::Operation, b: &mut Block) -> Result<()> {
    match *operation {
        ib::Operation::ThrowError => b.expr("panic!()".to_owned()),
        ib::Operation::Declare { ref var } => b.expr(format!("let {}", var)),
        ib::Operation::Assign { ref output_var, ref value, declare: false } =>
            b.assign(output_var.string(), build_expr(value)?.into()),
        ib::Operation::Assign { ref output_var, ref value, declare: true } =>
            b.let_assign(output_var.string(), build_expr(value)?.into()),
        ib::Operation::AddCount(ref var) =>
            b.assign("count".to_owned(), format!("count + {}", var)),
        ib::Operation::Block(ref inner_block) =>
            b.inline(build_block(inner_block)?),
        ib::Operation::ControlFlow { ref input_var,
                                     variant: ib::ControlFlowVariant::ForEachArray {
                                         ref loop_index_var, ref loop_value_var,
                                         ref inner } } => {
            b.for_(
                format!("&({}, ref {}) in {}.iter().enumerate()",
                        loop_index_var, loop_value_var, input_var),
                build_block(inner)?
            );
        }
        ib::Operation::ControlFlow { ref input_var,
                                     variant: ib::ControlFlowVariant::MatchUnionTag {
                                         ref cases, ref default, ref enum_type } } => {
            let mut cases = cases.iter()
                .map(|case| {
                    build_block(&case.block).map(|block| {
                        let mut ib = Block::new();
                        ib.inline(block);

                        MatchCase {
                            pattern: format!("TODO::{}", case.variant_name.pascal()),
                            block: ib,
                        }
                    })
                })
                .collect::<Result<_>>()?;

            b.match_(input_var.string(), cases);
        }
        _ => (),
    }

    Ok(())
}

pub fn build_expr(expr: &ib::Expr) -> Result<String> {
    let res = match *expr {
        ib::Expr::InputData => "input".to_owned(),
        ib::Expr::Var(ref var) => var.0.clone(),
        ib::Expr::Literal(ib::Literal::Number(ref num)) => num.clone(),
        ib::Expr::ContainerField { ref input_var, ref field } =>
            format!("{}.{}", input_var, field.pascal()),
        ib::Expr::ArrayLength(ref array) =>
            format!("{}.len()", array),
        _ => unimplemented!(),
    };
    Ok(res)
}
