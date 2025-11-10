# spring-controller-pest

* Crates: https://crates.io/crates/spring_controller_parser
* Docs: 

> A Rust crate that parses Java Spring `@Controller` and `@RestController` classes and their request-mapping method using pest.

---

## Overview

**Project name:** `spring-controller-pest`


### Technical description

The parser uses the pest parsing library to analyze Java Spring controller files by recognizing structural patterns and not full Java syntax. It identifies controller annotations (@Controller, @RestController), class declarations, and mapping annotations (@RequestMapping, @GetMapping, etc.), along with method signatures.

During parsing, pest matches these tokens using defined grammar rules, then the library builds an AST (It can be Controller, ControllerMethod structs) containing the class name, mappings, and method details.

### What is parsed

- Controller annotations: `@Controller`, `@RestController`
- Class name
- Class-level mapping: `@RequestMapping("/api")`
- Method-level mappings: `@GetMapping`, `@PostMapping`, etc.
- Method signatures

### How parsing results are used

The extracted AST can be used to:
- Generate documentation or OpenAPI specifications  
- Check endpoint coverage  
- Build mock API servers  


## Grammar

```pest
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }

controller_file = { SOI ~ controller_block* ~ EOI }

mapping_annotation = { 
    "@RequestMapping" ~ mapping_args? |
    "@GetMapping" ~ mapping_args? |
    "@PostMapping" ~ mapping_args? |
    "@DeleteMapping" ~ mapping_args? |
    "@PatchMapping" ~ mapping_args? |
    "@PutMapping" ~ mapping_args?
}
mapping_args = { "(" ~ (!")" ~ ANY)* ~ ")" }

identifier = @{ (ASCII_ALPHANUMERIC | "_")+ }
class_decl = { "class" ~ identifier }
method = {
    ( "public" | "protected" | "private" ) ~
    (!"(" ~ ANY)* ~
    "(" ~ (!")" ~ ANY)* ~ ")" ~
    WHITESPACE* ~
    "{" ~ (!"}" ~ ANY)* ~ "}"
}

controller_annotation = { "@Controller" | "@RestController" }

controller_block = {
    controller_annotation ~
    (!class_decl ~ ANY)* ~
    class_decl ~
    (!"}" ~ method)* ~
    "}"
}

```