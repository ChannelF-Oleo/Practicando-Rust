# 🦀 Mis Primeras Prácticas en Rust

Este repositorio contiene varios proyectos pequeños con los que practico conceptos básicos de Rust (entrada/salida, control de flujo, manejo de errores, crates, estructuras y más).

---

## Tecnologías

- Lenguaje: Rust
- Herramienta de compilación: Cargo

---

## Proyectos incluidos

Los proyectos están en carpetas independientes en la raíz del repositorio:

- `hello_cargo` — Ejemplo "Hola mundo" y verificación del entorno.
- `adivinanza_juego` — Juego para adivinar un número (usa `rand`).
- `ahorcado` — Implementación básica del juego del ahorcado.
- `calculadora` — Calculadora de consola (suma, resta, mult, div).
- `ventas` — Simulación sencilla de operaciones sobre productos/ventas.

---

## Cómo ejecutar

1. Instala Rust (si no lo tienes):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Compilar y ejecutar un proyecto (ejemplo `calculadora`):

```bash
cd calculadora
cargo run
```

Repite `cd <proyecto>` + `cargo run` para los demás proyectos.

---

## Notas

- Cada carpeta es un proyecto independiente (no es un workspace cargo).
- Si quieres que agregue un `Cargo.toml` de workspace o scripts para ejecutar todo, dímelo y lo preparo.

---

Archivo actualizado automáticamente.
