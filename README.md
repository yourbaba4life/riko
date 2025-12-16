# Riko


<p align="center">
  <img src="assets/riko.png" alt="Riko the Ermine" width="200">
</p>

A small, blazingly fast interpreted language. Cute name, serious intent.

---

## Roadmap

## Phase 1: The Lexer (Lexical Analysis)*Goal: Convert a raw string of code into a stream of meaningful "Tokens."*

* [x] **Define the `Token` Enum:** Create a robust Enum in Rust to represent keywords, operators, and literals (e.g., `Token::Let`, `Token::Int(5)`, `Token::Plus`).
* [ ] **Implement the `Lexer` Struct:** Use a `Peekable<Chars>` iterator to traverse the source code.
* [ ] **Handle Whitespace & Comments:** Skip characters that don't affect logic.
* [ ] **Identifier Recognition:** Implement logic to distinguish between keywords (like `fn`) and user-defined variables (like `my_var`).
* **Rust Skill focus:** Pattern matching on characters and `Iterators`.

---

##Phase 2: The Parser (Syntactic Analysis)*Goal: Turn the flat list of Tokens into a hierarchical Tree (Abstract Syntax Tree - AST).*

* [ ] **Define the AST Nodes:** Use Enums and Structs to represent the grammar (e.g., `Expression::Binary`, `Statement::Let`).
* [ ] **Implement Recursive Descent:** Write functions that call each other to handle grammar rules.
* [ ] **Implement Pratt Parsing:** Specifically for mathematical expressions to handle "Operator Precedence" (making sure `2 + 3 * 4` equals `14`, not `20`).
* [ ] **Error Handling:** Create a custom `ParseError` type to report exactly where the user made a typo.
* **Rust Skill focus:** Recursive data structures and `Box<T>` (for heap allocation of recursive types).

---

## Phase 3: The Evaluator (Tree-Walk Interpreter)*Goal: Walk the AST and execute the logic immediately.*

* [ ] **Define the `Object` System:** Create an Enum to represent your language's runtime values (Integer, Boolean, Null, Function).
* [ ] **The Eval Function:** Create a recursive function that takes an AST node and returns an `Object`.
* [ ] **Basic Arithmetic:** Implement addition, subtraction, etc., by matching on AST nodes.
* [ ] **Conditionals:** Implement `if/else` logic.
* **Rust Skill focus:** Deep understanding of `Enums` and result handling.

---

## Phase 4: Environment & Scope*Goal: Support variables and functions.*

* [ ] **The Environment Struct:** Use a `HashMap` to map variable names to `Object` values.
* [ ] **Nested Scopes:** Implement a system where inner scopes can see outer variables, but not vice versa.
* [ ] **Function Literals:** Store the function's parameters and body in the Environment.
* [ ] **Closures:** Handle functions that "capture" variables from their parent scope.
* **Rust Skill focus:** `Rc<RefCell<Environment>>` to handle shared, mutable ownership of scopes.

---

## Phase 5: The Virtual Machine (Compiling to Bytecode)*Goal: Move from "Tree-Walking" to a high-performance "Bytecode VM" (0 to 100 level).*

* [ ] **The Compiler:** Instead of evaluating the AST, write a compiler that emits "Opcodes" (e.g., `OP_ADD`, `OP_CONSTANT`).
* [ ] **The Bytecode Chunk:** A structure to hold a sequence of bytes and a constant pool.
* [ ] **The VM:** Build a Stack-Based VM that loops through instructions. Use a `Vec` as your stack.
* [ ] **Benchmarks:** Compare the speed of your Tree-Walk interpreter vs. your Bytecode VM.
* **Rust Skill focus:** Low-level memory layout, `Vec` optimization, and `match` performance.

---

##Phase 6: Quality of Life (The "Finish")* [ ] **The REPL:** Create a "Read-Eval-Print Loop" so you can type code into your terminal and see results instantly.
* [ ] **Standard Library:** Add built-in functions like `print()`, `len()`, or `push()`.
* [ ] **File Execution:** Allow the VM to read and run `.yourlang` files from the command line.

---

*Riko the ermine — small but has bite.*
