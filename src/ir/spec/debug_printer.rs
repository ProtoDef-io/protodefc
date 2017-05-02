use std::fmt;
use std::fmt::Debug;
use super::Type;

fn pad_level(fmt: &mut fmt::Formatter, level: u64) -> Result<(), fmt::Error> {
    for _ in 0..level {
        fmt.write_str("    ")?;
    }
    Ok(())
}

pub fn print(typ: &Type, fmt: &mut fmt::Formatter, depth: u64) -> Result<(), fmt::Error> {
    pad_level(fmt, depth)?;
    fmt.write_str("Node( ")?;
    typ.variant.fmt(fmt)?;
    if typ.data.get_children().len() == 0 {
        fmt.write_str(" );\n")?;
    } else {
        fmt.write_str(" ) {\n")?;

        for child in typ.data.get_children().iter() {
            match child.try_borrow() {
                Ok(borrow) => {
                    print(&borrow, fmt, depth+1)?;
                }
                Err(_) => {
                    fmt.write_str("<borrowed>")?;
                }
            }
        }

        pad_level(fmt, depth)?;
        fmt.write_str("};\n")?;
    }
    Ok(())
}

impl fmt::Debug for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str("protodefc::IR {\n")?;
        print(self, fmt, 1)?;
        fmt.write_str("}")?;
        Ok(())
    }
}
