use std::fmt::{self, Display};
mod parse;

fn main() {
    println!("{}", build(include_str!("../example.la")).unwrap())
}

fn build(source: &str) -> Result<String, String> {
    let ctx = &mut Context::default();
    let code = Expr::parse(source)?.compile(ctx)?;
    Ok(include_str!("template.asm")
        .replace("$src", source)
        .replace("$main", &code)
        .replace("$code", &ctx.code))
}

const INDENT: &str = "    ";
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
            ($asm: expr, $cmt: expr) => {
                format!("{INDENT}{<16};{cmt}\n", $asm, $cmt)
            };
        }
        match self {
            Expr::Variable(name) => Ok(mnemonic!(
                format!("\tmov rax, {}\t", REGS[ctx.variable(name)?],)
                format!( "Load variable: {name}")
            )),
            Expr::Apply(la, arg) => Ok(format!(
                "{}\tmov rbx, rax\t; Argument: {arg}\n\tpush rbx\t\t; Migrate (protect from overwrite)\n{}\tpop rbx\t\t\t; Reinstate in argument from stack\n\tcall rax\t\t; Apply lambda: {la}\n",
                arg.compile(ctx)?,
                la.compile(ctx)?,
            )),
            Expr::Lambda(arg, body) => {
                let id = ctx.id();
                let original_env = ctx.env.clone();
                ctx.bind(arg);
                let lambda_abstract = &format!(
                    "LA.{id}:\n{}{}\tret\n\n",
                    format!(
                        "\t; Lambda Abstract: {self}\n\t; Environment {{ {} }}\n\tmov {}, rbx\t; Bind variable: {arg}\n",
                        ctx.env
                            .iter()
                            .enumerate()
                            .map(|(i, x)| format!("{x}: {}", REGS[i]))
                            .collect::<Vec<_>>()
                            .join(", "),
                        REGS[ctx.variable(arg)?]
                    ),
                    body.compile(ctx)?
                );
                ctx.code += lambda_abstract;
                ctx.env = original_env;
                Ok(format!("\tmov rax, LA.{id}\t; Store address of lambda\n"))
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
            Err(format!("undefine variable: {name}"))
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
