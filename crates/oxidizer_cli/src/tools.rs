// use std::process::Command;

// trait BuildCommand {
//     fn execute(&self) -> Result<(), String>;
// }

// struct Clang {
//     args: Vec<String>,
// }

// impl Clang {
//     fn new() -> Self {
//         Clang { args: Vec::new() }
//     }

//     fn compile(mut self, source: &str) -> Self {
//         self.args.push(source.to_string());
//         self
//     }

//     fn output(mut self, output: &str) -> Self {
//         self.args
//             .extend_from_slice(&["-o".to_string(), output.to_string()]);
//         self
//     }

//     fn optimize(mut self, level: u8) -> Self {
//         self.args.push(format!("-O{}", level));
//         self
//     }

//     fn shared(mut self) -> Self {
//         self.args.push("-shared".to_string());
//         self
//     }
// }

// impl BuildCommand for Clang {
//     fn execute(&self) -> Result<(), String> {
//         let output = Command::new("clang")
//             .args(&self.args)
//             .output()
//             .map_err(|e| e.to_string())?;

//         if output.status.success() {
//             Ok(())
//         } else {
//             Err(String::from_utf8_lossy(&output.stderr).to_string())
//         }
//     }
// }

// struct Cargo {
//     args: Vec<String>,
// }

// impl Cargo {
//     fn new() -> Self {
//         Cargo { args: Vec::new() }
//     }

//     fn new_project(mut self, name: &str) -> Self {
//         self.args = vec!["new".to_string(), name.to_string()];
//         self
//     }

//     fn build(mut self) -> Self {
//         self.args = vec!["build".to_string()];
//         self
//     }

//     fn run(mut self) -> Self {
//         self.args = vec!["run".to_string()];
//         self
//     }

//     fn test(mut self) -> Self {
//         self.args = vec!["test".to_string()];
//         self
//     }

//     fn release(mut self) -> Self {
//         self.args.push("--release".to_string());
//         self
//     }
// }

// impl BuildCommand for Cargo {
//     fn execute(&self) -> Result<(), String> {
//         let output = Command::new("cargo")
//             .args(&self.args)
//             .output()
//             .map_err(|e| e.to_string())?;

//         if output.status.success() {
//             Ok(())
//         } else {
//             Err(String::from_utf8_lossy(&output.stderr).to_string())
//         }
//     }
// }

// struct CMake {
//     args: Vec<String>,
// }

// impl CMake {
//     fn new() -> Self {
//         CMake { args: Vec::new() }
//     }

//     fn configure(mut self, source_dir: &str, build_dir: &str) -> Self {
//         self.args = vec![
//             "-S".to_string(),
//             source_dir.to_string(),
//             "-B".to_string(),
//             build_dir.to_string(),
//         ];
//         self
//     }

//     fn build(mut self, build_dir: &str) -> Self {
//         self.args = vec!["--build".to_string(), build_dir.to_string()];
//         self
//     }

//     fn install(mut self, build_dir: &str) -> Self {
//         self.args = vec!["--install".to_string(), build_dir.to_string()];
//         self
//     }

//     fn target(mut self, target: &str) -> Self {
//         self.args
//             .extend_from_slice(&["--target".to_string(), target.to_string()]);
//         self
//     }
// }

// impl BuildCommand for CMake {
//     fn execute(&self) -> Result<(), String> {
//         let output = Command::new("cmake")
//             .args(&self.args)
//             .output()
//             .map_err(|e| e.to_string())?;

//         if output.status.success() {
//             Ok(())
//         } else {
//             Err(String::from_utf8_lossy(&output.stderr).to_string())
//         }
//     }
// }

// fn main() -> Result<(), String> {
//     // Uso de Clang
//     Clang::new()
//         .compile("program.cpp")
//         .output("program")
//         .optimize(2)
//         .execute()?;

//     Clang::new()
//         .compile("lib1.cpp")
//         .compile("lib2.cpp")
//         .shared()
//         .output("libmy.so")
//         .execute()?;

//     // Uso de Cargo
//     Cargo::new().new_project("my_rust_project").execute()?;
//     Cargo::new().build().release().execute()?;
//     Cargo::new().test().execute()?;

//     // Uso de CMake
//     CMake::new().configure(".", "build").execute()?;
//     CMake::new().build("build").execute()?;
//     CMake::new().build("build").target("test").execute()?;
//     CMake::new().install("build").execute()?;

//     Ok(())
// }

use std::{collections::HashMap, process::Command};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Language {
    C,
    Cpp,
    Rust,
    CMake,
}

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

trait Compiler {
    fn execute_step(&self, step: &CompilationStep) -> Result<(), String>;
}

struct Clang;
impl Compiler for Clang {
    fn execute_step(&self, step: &CompilationStep) -> Result<(), String> {
        match step {
            CompilationStep::Compile(source) => println!("Clang: Compiling {}", source),
            CompilationStep::Output(output) => println!("Clang: Output to {}", output),
            CompilationStep::Optimize(level) => println!("Clang: Optimizing at level {}", level),
            CompilationStep::EmitLLVM => println!("Clang: Emitting LLVM IR"),
            CompilationStep::Shared => println!("Clang: Creating shared library"),
            _ => return Err("Unsupported step for Clang".to_string()),
        }
        Ok(())
    }
}

struct Cargo;
impl Compiler for Cargo {
    fn execute_step(&self, step: &CompilationStep) -> Result<(), String> {
        match step {
            CompilationStep::Compile(source) => println!("Cargo: Building {}", source),
            CompilationStep::Build => println!("Cargo: Building project"),
            CompilationStep::Test => println!("Cargo: Running tests"),
            CompilationStep::EmitLLVM => println!("Cargo: Emitting LLVM IR"),
            _ => return Err("Unsupported step for Cargo".to_string()),
        }
        Ok(())
    }
}

struct CMake;
impl Compiler for CMake {
    fn execute_step(&self, step: &CompilationStep) -> Result<(), String> {
        match step {
            CompilationStep::Configure(source, build) => {
                println!("CMake: Configuring {} to {}", source, build)
            }
            CompilationStep::Build => println!("CMake: Building project"),
            CompilationStep::Install(dir) => println!("CMake: Installing to {}", dir),
            CompilationStep::Target(target) => println!("CMake: Setting target to {}", target),
            _ => return Err("Unsupported step for CMake".to_string()),
        }
        Ok(())
    }
}

struct BuildSystem {
    compilers: HashMap<Language, Box<dyn Compiler>>,
}

impl BuildSystem {
    fn new() -> Self {
        let mut compilers: HashMap<Language, Box<dyn Compiler>> = HashMap::new();
        compilers.insert(Language::Rust, Box::new(Cargo));
        compilers.insert(Language::C, Box::new(Clang));
        compilers.insert(Language::Cpp, Box::new(Clang));
        compilers.insert(Language::CMake, Box::new(CMake));
        Self { compilers }
    }

    fn compile(&self, recipe: CompilationRecipe) -> Result<(), String> {
        let compiler = self
            .compilers
            .get(&recipe.language)
            .ok_or_else(|| format!("Unsupported language: {:?}", recipe.language))?;

        for step in &recipe.steps {
            compiler.execute_step(step)?;
        }
        Ok(())
    }
}
