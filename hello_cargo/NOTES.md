# Notas

Sobre el uso de Cargo.
Es el sistema de compilación y administrador de paquetes de Rust. Se encarga de compilar código, descargar dependencias y compilarlas.

## Comandos utilizados

### Crea un nuevo proyecto con Cargo:

```bash
cargo new nombre_proyecto
```

Creará un proyecto con:
- `src/`
- `Cargo.toml`
- `Cargo.lock`

### Construir y ejecutar con Cargo

Crea un archivo ejecutable en `target/debug/hello`.

```bash
cargo build
```

Si es que ya no existía, creará un archivo `Cargo.lock` en el directorio raíz. Este archivo se encargará de rastrear las versiones exactas de las dependencias del proyecto. Cargo administra su contenido, por lo que no es necesario hacer cambios en el archivo de forma manual.

Se puede llamar al ejecutable con:

```bash
./target/debug/hello_cargo
```

> [!TIP] Importante
> Es importante saber que estos comandos están pensados para ser usados en macOS o Linux. Para windows será necesario usar la extensión .exe que se creará automáticamente para cada archivo ejecutable.

#### Compilar y llamar al ejecutable resultante con 1 comando

Será más conveniente que ejecutar `cargo run` y luego usar la ruta completa del binario.

```bash
cargo run
```

### Comprobar código

Así nos aseguramos de que el código compile y no produce un ejecutable (útil si es que constantemente se verifica el trabajo).

```bash
cargo check
```

### Construir una versión de lanzamiento

```bash
cargo build --release
```

Se crea un ejecutable en `target/release`. Optimiza para que el código se ejecute más rápido ralentizan la compilación del programa y es por esto que hay dos perfiles:
1. Desarrollo: construir rápido y con frecuencia
2. Lanzamiento: construir el programa final, el que se entrega al usuario y que se ejecutará lo más rápido posibl
