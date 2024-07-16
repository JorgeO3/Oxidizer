use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Language {
    C,
    Cpp,
    Rust,
    Go,
}

// Enumeración de los pasos de compilación posibles
#[derive(Clone)]
pub enum CompilationStep {
    Compile(String),
    Output(String),
    Optimize(u8),
    EmitLLVM,
    Shared,
    Build,
    Test,
    Configure(String, String),
    Install(String),
    Target(String),
}

// Estructura para la receta de compilación
pub struct CompilationRecipe {
    language: Language,
    steps: Vec<CompilationStep>,
}

impl CompilationRecipe {
    pub fn new(language: Language) -> Self {
        Self {
            language,
            steps: Vec::new(),
        }
    }

    pub fn add_step(mut self, step: CompilationStep) -> Self {
        self.steps.push(step);
        self
    }
}

trait LanguageCompiler {
    fn execute_step(&self, recipe: CompilationStep) -> Result<()>;
    fn compile(&self, source: String) -> Result<()>;
    fn output(&self, target: String) -> Result<()>;
    fn test(&self) -> Result<()>;
    fn build(&self) -> Result<()>;
    fn optimize(&self, level: u8) -> Result<()>;
    fn emit_llvm(&self) -> Result<()>;
    fn shared(&self) -> Result<()>;
    fn configure(&self, option: String, value: String) -> Result<()>;
    fn install(&self, path: String) -> Result<()>;
    fn target(&self, target: String) -> Result<()>;
}

struct ClangCompiler;
impl LanguageCompiler for ClangCompiler {
    fn execute_step(&self, step: CompilationStep) -> Result<()> {
        use CompilationStep::*;
        match step {
            Compile(source) => self.compile(source),
            Output(target) => self.output(target),
            Optimize(level) => self.optimize(level),
            EmitLLVM => self.emit_llvm(),
            Shared => todo!(),
            Build => todo!(),
            Test => todo!(),
            Configure(_, _) => todo!(),
            Install(_) => todo!(),
            Target(_) => todo!(),
        }
    }

    fn compile(&self, source: String) -> Result<()> {
        todo!()
    }

    fn output(&self, target: String) -> Result<()> {
        todo!()
    }

    fn test(&self) -> Result<()> {
        todo!()
    }

    fn build(&self) -> Result<()> {
        todo!()
    }

    fn optimize(&self, level: u8) -> Result<()> {
        todo!()
    }

    fn emit_llvm(&self) -> Result<()> {
        todo!()
    }

    fn shared(&self) -> Result<()> {
        todo!()
    }

    fn configure(&self, option: String, value: String) -> Result<()> {
        todo!()
    }

    fn install(&self, path: String) -> Result<()> {
        todo!()
    }

    fn target(&self, target: String) -> Result<()> {
        todo!()
    }
}

pub struct Compiler {
    recipes: Vec<CompilationRecipe>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            recipes: Vec::new(),
        }
    }

    pub fn add_recipe(mut self, recipe: CompilationRecipe) -> Self {
        self.recipes.push(recipe);
        self
    }
}
