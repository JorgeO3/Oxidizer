#[derive(Clone)]
enum CompilationStep {
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Language {
    C,
    Cpp,
    Rust,
    CMake,
}

struct CompilationRecipe {
    language: Language,
    steps: Vec<CompilationStep>,
}

impl CompilationRecipe {
    fn new(language: Language) -> Self {
        let steps = Vec::new();
        Self { language, steps }
    }

    fn add_step(mut self, step: CompilationStep) -> Self {
        self.steps.push(step);
        self
    }
}

struct Clang;

struct Cargo;

struct CMake;
