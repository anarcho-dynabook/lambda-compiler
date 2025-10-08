mod parse;

fn main() {
    println!(
        "{}",
        build(r#"(\m.\n.\f.m (n f)) (\f.\x.f (f (f x))) (\f.\x.f (f x))"#).unwrap()
    )
}

fn build(source: &str) -> Result<String, String> {
    let ctx = &mut Context::default();
    let code = Expr::parse(source)?.compile(ctx)?;
    Ok(include_str!("template.asm")
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
        match self {
            Expr::Variable(name) => Ok(format!(
                "\tmov rax, {}\t; load variable `{name}`\n",
                REGS[ctx.variable(name)?],
            )),
            Expr::Apply(la, arg) => Ok(format!(
                "\t; == arguments ==\n{}\tmov rbx, rax\n\t; == Lambda abstract ==\n{}\tcall rax\n",
                arg.compile(ctx)?,
                la.compile(ctx)?,
            )),
            Expr::Lambda(arg, body) => {
                let id = ctx.id();
                ctx.env.push(arg.clone());
                let original_env = ctx.env.clone();
                let lambda_abstract = &format!(
                    "LA.{id}:\n{}{}\tret\n\n",
                    format!("\tmov {}, rbx\t; Bind variable\n", REGS[ctx.variable(arg)?]),
                    body.compile(ctx)?
                );
                ctx.code += lambda_abstract;
                ctx.env = original_env;
                Ok(format!("\tlea rax, [rel LA.{id}]\n"))
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
}
