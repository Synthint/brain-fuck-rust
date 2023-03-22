extern crate proc_macro;
use proc_macro::{TokenStream};
//use quote::quote;



#[proc_macro]
pub fn brain_fuck(body : TokenStream) -> TokenStream {

    let tokens: Vec<char> = body.to_string()
                                .chars()
                                .filter(
                                    |c| !c.is_whitespace()
                            
                                ).collect();

    let mut commands: Vec<String> = Vec::new();
    let mut loop_validator = 0; 
    // counts up for open loop, down for close, must be 0 by end of token stream
    
    commands.push("let mut tape: Vec<u8> = vec![0; 255];".to_string());
    commands.push("let mut head_pos : u8 = 0;".to_string());
    commands.push("let mut read_line = String::new();;".to_string());

    for tok in tokens {
        match tok {
            '+' => {commands.push("tape[head_pos as usize] = tape[head_pos as usize].wrapping_add(1);".to_string());}
            '-' => {commands.push("tape[head_pos as usize] = tape[head_pos as usize].wrapping_sub(1);".to_string());}
            '>' => {commands.push("head_pos = head_pos.wrapping_add(1);".to_string());}
            '<' => {commands.push("head_pos = head_pos.wrapping_sub(1);".to_string());}
            ',' => {
                commands.push("std::io::stdin().read_line(&mut read_line).unwrap();".to_string());
                commands.push("tape[head_pos as usize] = read_line.chars().nth(0).unwrap() as u8;".to_string());
                }
            '.' => {commands.push("print!(\"{}\",tape[head_pos as usize] as char);".to_string());}
            '[' => {
                loop_validator += 1;
                commands.push("while tape[head_pos as usize] != 0 {".to_string());
            }
            ']' => {
                commands.push("}".to_string());
                loop_validator -= 1;
            }
            _ => {panic!("Unexpected token: {}, please only use the following tokens:
            > = increases memory pointer, or moves the pointer to the right 1 block.
            < = decreases memory pointer, or moves the pointer to the left 1 block.
            + = increases value stored at the block pointed to by the memory pointer
            - = decreases value stored at the block pointed to by the memory pointer
            [ = start a loop.
            ] = if block currently pointed to's value is not zero, jump back to the corresponding [
            , = input 1 character.
            . = print 1 character to the console
            ",tok)}
        };
    }

    if loop_validator != 0 {panic!("loops invalid, check that all [ have a corresponding ]")}

    return commands.join("\n").parse().unwrap();

}
