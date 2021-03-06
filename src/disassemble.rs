use std::io::{self, Write};
use crate::chunk::Chunk;


pub fn disassamble<W>(mut out: W, chunk: &Chunk, name: &str) -> io::Result<()>
where W: Write {
    writeln!(out, "== {} ==", name)?;
    
    let mut offset = 0;
    while offset < chunk.instructions().len() {
        disassamble_instruction(&mut out, chunk, offset, "")?;
        offset += 1;
    }
    Ok(())
}

pub fn disassamble_instruction<W>(mut out: W, chunk: &Chunk, offset: usize, more: &str)
-> io::Result<()> where W: Write {
    let line_string = {
        let lines = chunk.instruction_lines();
        if offset == 0 || lines[offset] != lines[offset - 1] {
            lines[offset].to_string()
        } else {
            "  |".to_string()
        }
    };
    let op = format!("{}", chunk.instructions()[offset]);
    writeln!(out, "{:04} {:>4} {:<20} {}", offset, line_string, op, more)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Instruction;
    use indoc::indoc;

    
    /// Trim the end of each line of the given string.
    fn trim_line_ends(s: &str) -> String {
        s.lines().map(|line| line.trim_end()).collect::<Vec<_>>().join("\n")
        + (if s.ends_with('\n') { "\n" } else { "" })
    }

    #[test]
    fn test_disassamble() {
        let mut chunk = Chunk::new();
        chunk.write(Instruction::Return, 123);
        chunk.write(Instruction::Constant(0), 123);

        let mut out = Vec::new();
        disassamble(&mut out, &chunk, "test").expect("disassamble failed");
        let out = String::from_utf8(out).expect("output is not utf8");
        let out = trim_line_ends(&out);

        assert_eq!(
            out,
            indoc!("
                == test ==
                0000  123 Return
                0001    | Constant(0)
            ")
        );
    }
}
