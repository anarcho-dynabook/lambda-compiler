mod parse;

fn main() {
    println!("{}", build(r#"(\x.x x) (\x. x)"#).unwrap())
}

fn build(source: &str) -> Result<String, String> {
    let ctx = &mut Context::default();
    let code = Expr::parse(source)?.compile(ctx)?;
    Ok(format!(
        ".text:\n\talign 16\n\tglobal _main\n_main:\n{code}\n\tmov rdi, rax\n\tmov rax, 0x2000001\n\tsyscall\n\n{}",
        ctx.code
    ))
}

const REGS: [&str; 13] = [
    "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
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
            Expr::Variable(name) => {
                if let Some(reg) = REGS.get(ctx.variable(name)?) {
                    Ok(format!("\tmov rax, {reg}\n"))
                } else {
                    Ok(format!("\tpop rax\n"))
                }
            }
            Expr::Apply(la, arg) => Ok(format!(
                "{}\tmov rdx, rax\n{}\tcall rax\n",
                arg.compile(ctx)?,
                la.compile(ctx)?,
            )),
            Expr::Lambda(arg, body) => {
                let id = ctx.id();
                let frame = &mut ctx.clone();
                frame.env.push(arg.clone());
                ctx.code += &format!(
                    "LA.{id}:\n{}{}\tret\n\n",
                    if let Some(reg) = REGS.get(frame.variable(arg)?) {
                        format!("\tmov {reg}, rdx\n")
                    } else {
                        format!("\tpush rdx\n")
                    },
                    body.compile(frame)?
                );
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
