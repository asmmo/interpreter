use std::collections::{linked_list::LinkedList, HashMap};
type InstructionsMap<'a> = HashMap<
    &'a str,
    (
        Box<dyn Fn(&mut LinkedList<i64>, &mut HashMap<&'a str, i64>, &'a str, Option<&&'a str>)>,
        usize,
    ),
>;
pub struct ByteCode<'a> {
    pub(crate) op_stack: LinkedList<i64>,
    pub(crate) registered_vars: HashMap<&'a str, i64>,
    pub(crate) instructions_map: InstructionsMap<'a>,
}

impl<'a> ByteCode<'a> {
    pub(crate) fn new() -> ByteCode<'a> {
        ByteCode {
            op_stack: LinkedList::new(),
            registered_vars: HashMap::new(),
            instructions_map: ByteCode::default_instructions_map(),
        }
    }
    pub fn evaluate_byte_code(code_block: &'a str) -> i64 {
        let mut byte_code = ByteCode::new();
        let cmds = byte_code.parse_cmds_from_block(code_block);
        byte_code.execute_cmds(cmds, 0);

        assert_eq!(
            byte_code.op_stack.len(),
            1,
            "Ill formed byte code. The stack of the byte code should have one element"
        );
        byte_code
            .op_stack
            .pop_back()
            .unwrap_or_else(|| panic!("INVALID_CODE"))
    }
    pub(crate) fn parse_cmds_from_block(&mut self, code_block: &'a str) -> Vec<&'a str> {
        let mut cmds = code_block
            .split('\n')
            .map(|cmd| cmd.trim_start_matches(char::is_whitespace))
            .map(|cmd| cmd.trim_end_matches(char::is_whitespace))
            .filter(|cmd| !cmd.is_empty())
            .collect::<Vec<&str>>();

        if cmds.is_empty() || cmds[cmds.len() - 1] != "RETURN_VALUE" {
            panic!(
                "Invalid byte code. The last command must be \'RETURN_VALUE\' while it is {}",
                cmds[cmds.len() - 1]
            );
        }
        cmds.pop();
        if cmds.is_empty() {
            panic!("Invalid byte code. The code block must contain at least one command before \"RETURN_VALUE\" command");
        }
        cmds
    }
    fn execute_cmds(&mut self, cmds: Vec<&'a str>, starting_index: usize) {
        for (index, cmd) in cmds.iter().enumerate() {
            if index > starting_index {
                self.parse_and_execute_cmd(cmd, cmds.get(index - 1))
            } else {
                self.parse_and_execute_cmd(cmd, None)
            }
        }
    }
    pub(crate) fn parse_and_execute_cmd(&mut self, cmd: &'a str, prev_cmd: Option<&&'a str>) {
        if cmd.is_empty() {
            panic!("Empty command")
        }
        let cmd_parts = cmd.split(' ').collect::<Vec<&str>>();
        let (op, operands_number) = self
            .instructions_map
            .get(cmd_parts[0])
            .as_ref()
            .unwrap_or_else(|| panic!("INVALID COMMAND:\'{}\'.", cmd));
        assert_eq!(cmd_parts.len(), *operands_number);
        op(&mut self.op_stack, &mut self.registered_vars, cmd, prev_cmd);
    }
    pub fn extend_instructions(
        &mut self,
        instruction_name: &'a str,
        instruction_operation: impl Fn(&mut LinkedList<i64>, &mut HashMap<&'a str, i64>, &'a str, Option<&&'a str>)
            + 'static,
        operands_number: usize,
    ) {
        self.instructions_map.insert(
            instruction_name,
            (Box::new(instruction_operation), operands_number),
        );
    }
    fn get_var_name(cmd: &str) -> &str {
        let second_cmd_part = cmd.split(' ').collect::<Vec<&str>>()[1];
        if second_cmd_part.len() < 3 {
            panic!("INVALID COMMAND:\'{}\'.", cmd)
        }
        let cmd_chars = second_cmd_part.chars().collect::<Vec<char>>();
        assert!(
            cmd_chars[0] == '\'' && cmd_chars[cmd_chars.len() - 1] == '\'',
            "The variable name must be enclosed in single quotes while it is {}",
            second_cmd_part
        );
        &second_cmd_part[1..second_cmd_part.len() - 1]
    }
    fn default_instructions_map() -> InstructionsMap<'a> {
        let mut default_instruction_map: InstructionsMap = HashMap::new();
        default_instruction_map.insert("ADD", (Box::new(|op_stack, _, _,_| {
            let var1: i64 = op_stack.pop_back().unwrap_or_else(
                || panic!("Invalid byte code. ADD command must be preceded by two numbers reading or loading. It is not preceded by any command"),
            );
            let var2: i64 = op_stack.pop_back().unwrap_or_else(
                || panic!("Invalid byte code. ADD command must be preceded by two numbers reading or loading. It is preceded by one only"),
            );
            op_stack.push_back(var1 + var2);

        }), 1));
        default_instruction_map.insert("MULTIPLY", (Box::new(|op_stack, _, _,_| {
            let var1: i64 = op_stack.pop_back().unwrap_or_else(
                || panic!("Invalid byte code. MULTIPLY command must be preceded by two numbers reading or loading. It is not preceded by any command"),
            );
            let var2: i64 = op_stack.pop_back().unwrap_or_else(
                || panic!("Invalid byte code. MULTIPLY command must be preceded by two numbers reading or loading. It is preceded by one only"),
            );
            op_stack.push_back(var1 * var2);

        }), 1));
        default_instruction_map.insert(
            "LOAD_VAL",
            (
                Box::new(|op_stack, _, cmd, _| {
                    let cmd_parts = cmd.split(' ').collect::<Vec<&str>>();
                    op_stack.push_back(cmd_parts[1].parse::<i64>().unwrap());
                }),
                2,
            ),
        );

        default_instruction_map.insert("WRITE_VAR", (Box::new(|op_stack, registered_vars, cmd,prev_cmd| {
            if let Some(prev_cmd) = prev_cmd {
                if !(prev_cmd.contains("LOAD_VAR") || prev_cmd.contains("LOAD_VAL")) {
                    panic!("Invalid byte code. WRITE_VAR command must be preceded by a number reading or loading. It is not preceded by any command");
                }
            } else {
                panic!("Invalid byte code. WRITE_VAR command must be preceded by a number reading or loading. It is preceded by one only");
            }
            let var_name = Self::get_var_name(cmd);
            registered_vars.insert(
                var_name,
                op_stack
                    .pop_back()
                    .unwrap_or_else(|| {
                        panic!(
                            "INVALID COMMAND\'{}\'.\nThe previous command for {} must be loading value command",
                            cmd,
                            cmd
                        )
                    })
            );
        }), 2));

        default_instruction_map.insert(
            "READ_VAR",
            (
                Box::new(|op_stack, registered_vars, cmd, _| {
                    let var_name = Self::get_var_name(cmd);
                    let var_value = registered_vars.remove(&var_name).unwrap_or_else(|| {
                        panic!(
                            "INVALID COMMAND\'{}\'.\nThe variable {} is not registered",
                            cmd, var_name
                        )
                    });
                    op_stack.push_back(var_value);
                }),
                2,
            ),
        );

        default_instruction_map
    }
}

#[cfg(test)]
mod tests;
