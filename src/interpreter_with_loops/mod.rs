use crate::interpreter::ByteCode;
use std::str::FromStr;

pub struct ByteCodeWithLoops<'a> {
    pub inner: ByteCode<'a>,
}

impl<'a> ByteCodeWithLoops<'a> {
    pub fn evaluate_byte_code(code_block: &'a str) -> i64 {
        let mut byte_code = ByteCodeWithLoops {
            inner: ByteCode::new(),
        };
        let cmds = byte_code.inner.parse_cmds_from_block(code_block);
        byte_code.evaluate_sub_blocks_from_line_number_n_times(&cmds, 0, 1);
        assert_eq!(
            byte_code.inner.op_stack.len(),
            1,
            "Ill formed byte code. The stack of the byte code should have one element"
        );
        byte_code
            .inner
            .op_stack
            .pop_back()
            .unwrap_or_else(|| panic!("INVALID_CODE"))
    }
    fn evaluate_sub_blocks_from_line_number_n_times(
        &mut self,
        cmds: &[&'a str],
        line_number: usize,
        n: usize,
    ) -> usize {
        let mut idx = line_number;
        for _ in 0..n {
            idx = line_number;
            while idx < cmds.len() {
                let cmd = cmds[idx];
                let cmd_parts = cmd.split(' ').collect::<Vec<&str>>();
                if cmd_parts.len() == 2 && cmd_parts[0] == "LOOP_START" {
                    idx += self.evaluate_sub_blocks_from_line_number_n_times(
                        cmds,
                        idx + 1,
                        usize::from_str(cmd_parts[1]).unwrap(),
                    );
                } else if cmd == "LOOP_END" {
                    idx += 1;
                    break;
                    // }
                } else if idx > 0 {
                    self.inner.parse_and_execute_cmd(cmd, cmds.get(idx - 1));
                } else {
                    self.inner.parse_and_execute_cmd(cmd, None);
                }
                idx += 1;
            }
        }
        idx - line_number
    }
}

#[cfg(test)]
mod tests;
