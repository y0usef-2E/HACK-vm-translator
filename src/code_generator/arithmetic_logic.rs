use crate::utils::add_padding;

use super::{at, generate_mem_code_block, DEFAULT_PADDING};

pub fn generate_a_l_code_block(
    a_l_cmd: &str,
    filename: &str,
    jump_counter_ref: &mut usize,
    is_debug_option: bool,
    padding: usize,
) -> Vec<String> {
    let mut code_block: Vec<String> = vec![];

    *jump_counter_ref += 1;

    if is_debug_option {
        let comment = format!("// {}", a_l_cmd);
        code_block.push("\n".to_string());
        code_block.push(comment);
    }

    let mut pop1 = generate_mem_code_block(
        "pop",
        "general",
        13,
        filename,
        is_debug_option,
        DEFAULT_PADDING,
    );
    code_block.append(&mut pop1);

    // `neg` and `not` operate on one value only, so there is no need for popping a second value from the stack
    if a_l_cmd != "neg" && a_l_cmd != "not" {
        let mut pop2 = generate_mem_code_block(
            "pop",
            "general",
            14,
            filename,
            is_debug_option,
            DEFAULT_PADDING,
        );
        code_block.append(&mut pop2);
    }

    code_block.push(at(13)); // A = 13
    code_block.push("D = M".to_string()); // store content in D

    let mut temp_vec: Vec<String>;
    let label_if_true = format!("true_expression{}", *jump_counter_ref);
    let label_if_false = format!("false_expression{}", *jump_counter_ref);
    match a_l_cmd {
        "add" => {
            temp_vec = vec![
                at(14),                  // go to temp 2
                "D = D + M".to_string(), // D  = D + value of temp2
                "M = D".to_string(),     // replace temp 2 with the value of temp2+temp1
            ];
        }
        "sub" => {
            temp_vec = vec![
                at(14),                  // go to temp 2
                "D = M - D".to_string(), // D  = temp2 - temp1
                "M = D".to_string(),     // replace temp 2 with the value of temp2 - temp1
            ];
        }
        "neg" => {
            temp_vec = vec![
                "D = -D".to_string(), // D  = - temp1
                at(14),               // go to temp 2
                "M = D".to_string(),  // replace temp 2 with the value of -temp1
            ];
        }
        "and" => {
            temp_vec = vec![
                at(14),                  // go to temp 2
                "D = M & D".to_string(), // D  = temp2 & temp1
                "M = D".to_string(),     // replace temp 2 with the value of temp2 & temp1
            ];
        }
        "or" => {
            temp_vec = vec![
                at(14),                  // go to temp 2
                "D = M | D".to_string(), // D  = temp2 | temp1
                "M = D".to_string(),     // replace temp 2 with the value of temp2 | temp1
            ];
        }
        "not" => {
            temp_vec = vec![
                "D = !D".to_string(), // D = !temp1
                at(14),               // go to temp 2
                "M = D".to_string(),  // replace temp 2 with the value of !temp1
            ];
        }
        "eq" => {
            temp_vec = vec![
                at(14),                  // go to temp 2
                "D = M - D".to_string(), // D  = temp2 - temp1 (order doesn't matter since we're checking for inequality with 0)
                at(&label_if_true),
                "D;JEQ".to_string(),
                at(14),
                "M = 0".to_string(),
                at(&label_if_false),
                "0;JMP".to_string(),
                format!("({label_if_true})"),
                at(14),
                "M = -1".to_string(),
                format!("({label_if_false})"),
            ];
        }
        "gt" | "lt" => {
            code_block.push(at(14));

            if a_l_cmd == "lt" {
                code_block.push("D = D - M".to_string());
            // D  = temp1 - temp2 (if D>0 then temp2<temp1)
            } else {
                code_block.push("D = M - D".to_string());
                // D  = temp2 - temp1 (if D>0 then temp2>temp1)
            }

            // the rest is the same
            temp_vec = vec![
                at(&label_if_true),
                "D;JGT".to_string(), // jump to (greater) only if D>0
                at(14),              // go to temp 2 (this code will be executed if NOT[D > 0])
                "M = 0".to_string(), // set temp2 to false
                at(&label_if_false),
                "0;JMP".to_string(),
                format!("({label_if_true})"),
                at(14),               // go to temp 2
                "M = -1".to_string(), // set temp2 to true since D>0
                format!("({label_if_false})"),
            ];
        }
        _ => {
            eprintln!(
                "[ERROR] {} cannot be cannot be recognized as an arithmetic or logical command.",
                a_l_cmd
            );
            panic!();
        }
    }
    code_block.append(&mut temp_vec);

    let mut push_temp_2 = generate_mem_code_block(
        "push",
        "general",
        14,
        filename,
        is_debug_option,
        DEFAULT_PADDING,
    );
    code_block.append(&mut push_temp_2);

    code_block = if is_debug_option && padding != 0 {
        code_block.iter().map(|s| add_padding(s, padding)).collect()
    } else {
        code_block
    };
    return code_block;
}
