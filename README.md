# yaml-ast

A pure Rust YAML 1.2.2 parser ([Spec](https://yaml.org/spec/1.2.2/))

---

#### ⚠️ Notice

This project is currently under very active development. It is **not** production-ready or even spec-compliant (yet).
The official state is: Exploration / experimentation phase. It eventually aims to provide deeper control over the YAML
AST and the event-based emitter/parser. It is aimed at projects which require full insight into the YAML structure for
advanced topics like automatic code generations via derive macros.
