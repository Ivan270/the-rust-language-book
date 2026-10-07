fn main() {
    // con el ! se llama a una macro de Rust, sin el ! sería una función
    // Macros no siempre siguen las mismas reglas que las funciones
    println!("¡Hola mundo!");
}

// Compilar y ejecutar son pasos separados
// Primero se compila usando el compilador de rust: `$ rustc main.rs`
// Luego Rust genera un ejecutable binario (se puede ver en consola `$ ls`)
// Archivo ejecutable se puede compartir, el receptor no necesita tener Rust instalado en su sistema
