use indexmap::IndexMap;

fn main() {
    println!("Hello, world!");
}

const REGS: [&str; 13] = [
    "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
];

#[derive(Clone, Debug)]
enum Expr {
    Variable(String),
    Apply(Box<Expr>, Box<Expr>),
    Lambda(String, Box<Expr>),
}

#[derive(Clone, Debug)]
struct Context {
    id: usize,
    code: String,
    env: Vec<String>,
}

impl Expr {
    fn compile(&self, ctx: &mut Context) -> Option<String> {
        match self {
            Expr::Variable(name) => {
                if let Some(reg) = REGS.get(ctx.variable(name)?) {
                    Some(format!("\tmov rax, {reg}\n"))
                } else {
                    Some(format!("\tpop rax\n"))
                }
            }
            Expr::Apply(la, arg) => Some(format!(
                "{}\tmov rdx, rax\n{}\tcall rax\n",
                arg.compile(ctx)?,
                la.compile(ctx)?,
            )),
            Expr::Lambda(arg, body) => {
                let id = ctx.id();
                let frame = &mut ctx.clone();
                frame.env.push(arg.clone());
                ctx.code += &format!("LA.{id}:\n{}\tret\n\n", body.compile(frame)?);
                Some(format!("\tmov rax, LA.{id}\n"))
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

    fn variable(&mut self, name: &str) -> Option<usize> {
        self.env.iter().position(|x| x == name)
    }
}
