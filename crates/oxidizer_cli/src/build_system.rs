struct BuildSystem;
impl<T, const N: usize> From<[T; N]> for BuildSystem {
    fn from(value: [T; N]) -> Self {
        todo!()
    }
}

trait SystemBuilder {
    fn new();
    fn compile();
    fn output();
    fn optimize();
    fn emit_llvm();
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
    language: String,
    steps: Vec<CompilationStep>,
}

impl CompilationRecipe {
    pub fn new(language: &str) -> Self {
        CompilationRecipe {
            language: language.to_string(),
            steps: Vec::new(),
        }
    }

    pub fn add_step(mut self, step: CompilationStep) -> Self {
        self.steps.push(step);
        self
    }
}
