#[allow(dead_code)]

use std::{
    env, fs::File, io::{Read, Write},
    process::{exit, Command}
};

use colored::Colorize;

macro_rules! error_msg {
    ($($arg:tt)*) => {
        eprintln!("{} {}", "[Error]:".red(), format_args!($($arg)*));
    }
}

#[derive(PartialEq)]
enum Operations {
    Push,
    
    Add,
    Sub,
    
    Equal,
    
    Dump,
}

fn simulate_program(program: Vec<(Operations, Option<i64>)>) {
    let mut stack: Vec<i64> = Vec::new();
    for op in program {
        match op.0 {
            Operations::Push => {
                stack.push(op.1.unwrap());
            }
            
            Operations::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b + a);
            }
            Operations::Sub => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            }
            
            Operations::Equal => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((b == a) as i64);
            }
            
            Operations::Dump => {
                println!("{}", stack[stack.len() - 1]);
            }
        }
    }
}

fn compile_program(program: Vec<(Operations, Option<i64>)>, output_file: &String) {
    let mut file: File = File::create(output_file).unwrap();
    let _ = file.write(b"section .text\n");
    let _ = file.write(b"dump:\n");
    let _ = file.write(b"    mov     r9, -3689348814741910323\n");
    let _ = file.write(b"    sub     rsp, 40\n");
    let _ = file.write(b"    mov     BYTE [rsp+31], 10\n");
    let _ = file.write(b"    lea     rcx, [rsp+30]\n");
    let _ = file.write(b".L2:\n");
    let _ = file.write(b"    mov     rax, rdi\n");
    let _ = file.write(b"    lea     r8, [rsp+32]\n");
    let _ = file.write(b"    mul     r9\n");
    let _ = file.write(b"    mov     rax, rdi\n");
    let _ = file.write(b"    sub     r8, rcx\n");
    let _ = file.write(b"    shr     rdx, 3\n");
    let _ = file.write(b"    lea     rsi, [rdx+rdx*4]\n");
    let _ = file.write(b"    add     rsi, rsi\n");
    let _ = file.write(b"    sub     rax, rsi\n");
    let _ = file.write(b"    add     eax, 48\n");
    let _ = file.write(b"    mov     BYTE [rcx], al\n");
    let _ = file.write(b"    mov     rax, rdi\n");
    let _ = file.write(b"    mov     rdi, rdx\n");
    let _ = file.write(b"    mov     rdx, rcx\n");
    let _ = file.write(b"    sub     rcx, 1\n");
    let _ = file.write(b"    cmp     rax, 9\n");
    let _ = file.write(b"    ja      .L2\n");
    let _ = file.write(b"    lea     rax, [rsp+32]\n");
    let _ = file.write(b"    mov     edi, 1\n");
    let _ = file.write(b"    sub     rdx, rax\n");
    let _ = file.write(b"    xor     eax, eax\n");
    let _ = file.write(b"    lea     rsi, [rsp+32+rdx]\n");
    let _ = file.write(b"    mov     rdx, r8\n");
    let _ = file.write(b"    mov     rax, 1\n");
    let _ = file.write(b"    syscall\n");
    let _ = file.write(b"    add     rsp, 40\n");
    let _ = file.write(b"    ret\n");
    let _ = file.write(b"global _start\n");
    let _ = file.write(b"_start:\n");
    for op in program {
        match op.0 {
            Operations::Push => {
                let _ = file.write(format!("    push {}\n", op.1.unwrap()).as_bytes());
            }
            
            Operations::Add => {
                let _ = file.write(b"    pop rax\n");
                let _ = file.write(b"    pop rbx\n");
                let _ = file.write(b"    add rbx, rax\n");
                let _ = file.write(b"    push rbx\n");
            }
            Operations::Sub => {
                let _ = file.write(b"    pop rax\n");
                let _ = file.write(b"    pop rbx\n");
                let _ = file.write(b"    sub rbx, rax\n");
                let _ = file.write(b"    push rbx\n");
            }
            
            Operations::Equal => {
                let _ = file.write(b"    mov rcx, 0\n");
                let _ = file.write(b"    mov rdx, 1\n");
                let _ = file.write(b"    pop rax\n");
                let _ = file.write(b"    pop rbx\n");
                let _ = file.write(b"    cmp rax, rbx\n");
                let _ = file.write(b"    cmove rcx, rdx\n");
                let _ = file.write(b"    push rcx\n");
            }
            
            Operations::Dump => {
                let _ = file.write(b"    pop rdi\n");
                let _ = file.write(b"    call dump\n");
            }
        }
    }
    let _ = file.write(b"    mov rax, 60\n");
    let _ = file.write(b"    mov rdi, 0\n");
    let _ = file.write(b"    syscall\n");
}

fn lex(contents: String) -> Vec<(usize, String)> {
    let mut tokens: Vec<(usize, String)> = Vec::new();
    let mut line_num: usize = 1;
    for line in contents.lines() {
        for token in line.split_whitespace() {
            tokens.push((line_num, token.to_string()));
        }
        line_num += 1;
    }
    
    tokens
}

fn parse_tokens_as_op(program: &mut Vec<(Operations, Option<i64>)>, tokens: Vec<(usize, String)>) {
    for token in tokens {
        match token.1.as_str() {
            "+" => { program.push((Operations::Add, None)); }
            "-" => { program.push((Operations::Sub, None)); }
            
            "?=" => { program.push((Operations::Equal, None)); }
            
            "dump" => { program.push((Operations::Dump, None)); }
            
            _ => {
                match token.1.parse::<i64>() {
                    Ok(_) => {
                        program.push((Operations::Push, Some(token.1.parse().unwrap())));
                    }
                    Err(_) => {
                        error_msg!("Invalid token `{}` at line: {}", token.1, token.0);
                        exit(1);
                    }
                }
            }
        }
    }
}

fn load_program_from_file(file_path: String) -> Vec<(Operations, Option<i64>)> {
    let mut file: File = File::open(file_path).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    
    let tokens = lex(contents);
    let mut program: Vec<(Operations, Option<i64>)> = Vec::new();
    parse_tokens_as_op(&mut program, tokens);
    
    program
}

fn usage(program_file: &String) {
    println!("{} {} {}", "[Usage]:".yellow(), program_file, "<[SUBCOMMAND]> <[ARGS]>");
    println!("{}", "[Subcommand]:".blue());
    println!("    - build: <FILE> <EXEC> -> Compiles the program to a executable");
    println!("    - sim:   <FILE>        -> Simulates the program");
}

fn shift(args: &Vec<String>, _index: &mut usize) -> String {
    let last_index = *_index;
    *_index += 1;
    args[last_index].clone()
}

fn main() {
    let mut args_index = 0;
    let args: Vec<String> = env::args().collect();
    
    let program_file = shift(&args, &mut args_index);
    
    if args.len() < 2 {
        usage(&program_file);
        error_msg!("[Error]: Invalid subcommand");
        exit(1);
    }
    
    let subcommand = shift(&args, &mut args_index);
    match subcommand.as_str() {
        "build" => {
            if args.len() < 3 || args.len() > 4 {
                usage(&program_file);
                error_msg!("Invalid arguments!");
                exit(1);
            }
            
            let input = shift(&args, &mut args_index);
            let output = shift(&args, &mut args_index);
            let output_ext_s = output.clone() + ".s";
            let output_ext_o = output.clone() + ".o";
            
            let program = load_program_from_file(input);
            compile_program(program, &output_ext_s);
            
            let _ = Command::new("nasm").args(vec!["-felf64", &output_ext_s]).output();
            let _ = Command::new("ld").args(vec!["-o", &output, &output_ext_o]).output();
        }
        "sim" => {
            if args.len() < 3 || args.len() > 3 {
                usage(&program_file);
                error_msg!("[Error]: Invalid arguments!");
                exit(1);
            }
            
            let input = shift(&args, &mut args_index);
            let program = load_program_from_file(input);
            simulate_program(program);
        }
        
        _ => {
            usage(&program_file);
            error_msg!("[Error]: invalid subcommand `{}`", subcommand);
            exit(1);
        }
    }
}
