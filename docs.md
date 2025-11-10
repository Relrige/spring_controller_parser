# Spring Controller Parser Documentation

It is simple Spring Controller Parser crate

## Overview

The Spring Controller Parser is a parser that parses Java Spring controller classes using a Pest grammar. It extracts controller information such as class names, mappings  and annotated methods and represents them as AST.

## Features

* Parses Java Spring controller annotations such as `@Controller`, `@RestController`
* Extracts class-level mappings
* Extracts method-level mappings like annotation type, annotation arguments and aslo method headers
* Provides a clean AST-like representation using `Controller` and `ControllerMethod` structs

## Example

```rust
use spring_controller_parser::parse_controllers;

let source = r#"
@RestController
@RequestMapping("/test")
class TestController {
    @GetMapping("/test")
    public String test() {
        return "Test";
    }
}
"#;

let controllers = parse_controllers(source).unwrap();
```

## Public API Summary

### `parse_controllers(src: &str)`

Parses a Java source string and returns a AST of `Controller` 

### `Controller`

Represents a Spring controller class.

* `name`: Name of the class
* `class_mapping`: Optional class-level `@RequestMapping`
* `methods`: List of `ControllerMethod`

### `ControllerMethod`

Represents a mapped controller method.

* `annotation`: Type of mapping annotation (e.g. `GetMapping`)
* `annotation_args`: Arguments of the mapping annotation
* `header`: Extracted method header

## CLI Usage

The crate also includes a small CLI application:

```
spring_controller_parser parse <file>
```

