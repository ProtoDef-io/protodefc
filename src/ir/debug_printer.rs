use ::TypeContainer;

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
    typ.data.name.fmt(fmt)?;
    fmt.write_str(", ")?;
    typ.variant.fmt(fmt)?;
    if typ.data.children.len() == 0 {
        fmt.write_str(" );\n")?;
    } else {
        fmt.write_str(" ) {\n")?;

        for child in &typ.data.children {
            match child.0.try_borrow() {
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

impl fmt::Debug for TypeContainer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str("protodefc::IR {\n")?;
        match self.0.try_borrow() {
            Ok(borrow) =>
                print(&borrow, fmt, 1)?,
            Err(_) =>
                fmt.write_str("<borrowed>")?,
        }
        fmt.write_str("}")?;
        Ok(())
    }
}
