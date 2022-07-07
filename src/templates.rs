#![allow(dead_code)]

pub static CLANGD: &str = r#"
CompileFlags:                   
  Add: [-xc++, -Wall, -std=c++17]

InlayHints:
  Enabled: Yes
  ParameterNames: Yes
  DeducedTypes: Yes
"#;

pub static CLANG_FORMAT: &str = r#"
BasedOnStyle: LLVM
IndentWidth: 4
TabWidth: 4
AllowShortIfStatementsOnASingleLine: true
NamespaceIndentation: All
Language: Cpp
DerivePointerAlignment: false
PointerAlignment: Left
AlignAfterOpenBracket: true
AlignConsecutiveAssignments: true
"#;