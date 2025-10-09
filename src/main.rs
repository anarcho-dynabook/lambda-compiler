use parse::{BLANK, INDENT};
use std::fmt::{self, Display};
use std::io::{Read, Write, stdin, stdout};
use std::process::exit;

mod parse;

fn main() {
    macro_rules! error {
        ($value: expr) => {
            match $value {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("{err}");
                    exit(1)
                }
            }
        };
    }
    let code = {
        let mut buffer = String::new();
        error!(stdin().read_to_string(&mut buffer));
        buffer.trim().to_owned()
    };
    let output = error!(build(&code));
    error!(stdout().write_all(output.as_bytes()));
}

fn build(source: &str) -> Result<String, String> {
    let ctx = &mut Context::default();
    let code = Expr::parse(source)?.compile(ctx)?;
    Ok(include_str!("template.asm")
        .replace("$src", source)
        .replace("$main", &code)
        .replace("$code", &ctx.code))
}

const REGS: [&str; 12] = [
    "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
];

#[derive(Clone, Debug)]
pub enum Expr {
    Variable(String),
    Apply(Box<Expr>, Box<Expr>),
    Lambda(String, Box<Expr>),
}

#[derive(Clone, Debug, Default)]
struct Context {
    id: usize,
    code: String,
    env: Vec<String>,
}

impl Expr {
    fn compile(&self, ctx: &mut Context) -> Result<String, String> {
        macro_rules! mnemonic {
            ($asm: expr => $cmt: expr) => {
                match ($asm.to_string().as_str(), $cmt.to_string().as_str()) {
                    (BLANK, cmt) => format!("{INDENT};;; {cmt}\n"),
                    (asm, BLANK) => format!("{INDENT}{asm}\n"),
                    (asm, cmt) => format!("{INDENT}{asm:<20}; {cmt}\n"),
                }
            };
        }
        match self {
            Expr::Variable(name) => Ok(mnemonic!(
                format!("mov rax, {}", REGS[ctx.variable(name)?])
                => format!("Load variable `{name}`")
            )),
            Expr::Apply(la, arg) => Ok(format!(
                "{arg}{0}{1}{la}{2}{3}",
                mnemonic!("mov rbx, rax" => format!("Argument `{arg}`")),
                mnemonic!("push rbx"     => "Retract to stack"),
                mnemonic!("pop rbx"      => "(overwrite-guard)"),
                mnemonic!("call rax"     => format!("Apply `{la}`")),
                arg = arg.compile(ctx)?,
                la = la.compile(ctx)?,
            )),
            Expr::Lambda(arg, body) => {
                let id = ctx.id();
                let original_env = ctx.env.clone();
                ctx.bind(arg);

                let body = body.compile(ctx)?;
                let lambda_abstract = &format!(
                    "LA.{id}:\n{0}{1}{2}{body}{3}\n",
                    mnemonic!(BLANK => format!("Lambda Abstract `{self}`")),
                    mnemonic!(BLANK => format!("Environment {{ {} }}",
                        ctx.env
                            .iter()
                            .enumerate()
                            .map(|(i, x)| format!("{x}: {}", REGS[i].to_uppercase()))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )),
                    mnemonic!(
                        format!("mov {}, rbx", REGS[ctx.variable(arg)?])
                        => format!("Bind variable `{arg}`")
                    ),
                    mnemonic!("ret" => BLANK)
                );
                ctx.code += lambda_abstract;
                ctx.env = original_env;
                Ok(mnemonic!(format!("mov rax, LA.{id}") => "Absolute address (No PIE)"))
            }
        }
    }
}

impl Context {
    fn id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }

    fn variable(&mut self, name: &str) -> Result<usize, String> {
        if let Some(var) = self.env.iter().position(|x| x == name) {
            Ok(var)
        } else {
            Err(format!("undefine variable `{name}`"))
        }
    }

    fn bind(&mut self, name: &str) {
        if !self.env.contains(&name.to_string()) {
            self.env.push(name.to_string());
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Apply(a, b) => write!(f, "({a}{b})"),
            Expr::Variable(a) => write!(f, "{a}"),
            Expr::Lambda(a, b) => write!(f, "(λ{a}.{b})"),
        }
    }
}
