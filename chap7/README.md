## Modules & packages

Rust module system has the following features:

- packages (build and share crates)
- crates (tree of modules that produces a library/executable)
- modules + use: organize scope and paths
- paths: way of naming an item

**Crate**: smallest amount of code that the compiler considers at a time; crates can contain modules.

Two forms: binary - libary. Binary crates are program that can be compiled to an executable to run (they require a main); library crates don't compile to an executable but define functionality to be shared with multiple projects (ex: rand).

Crate root is the source file that the compiler stats from.

**Package**: bundle of one or more crates that probides a set of functionalities; it contains a Cargo.toml that describes how to build those crates.

### Modules

- crate root: compiler starts from a root file (src/lib.rs, sc/main.rs).
- declaring modules: happens in the crate root
  ```rust
    // src/lib.rs
    mod garden;
    // compiler will search the code in:
    // > inline (curly brackets after mod garden)
    // > src/garden.rs
    // > src/garden/mod.rs
  ```
- declaring submodules: any module declared outside of the crate root;
  ```rust
    // src/garden.rs
    mod vegetables;
    // compiler will search the code in:
    // > inline (curly brackets after mod garden)
    // > src/garden/vegetables.rs
    // > src/garden/vegetables/mod.rs
  ```
- paths to code in module: anything can be referenced, if privacy rules allows
  ```rust
  use crate::garden::vegetales::Asparagus;

  let asparagus = Asparagus::new();
  ```
- private vs public: code within a module is private, to declare it public `pub mod`
- use keyword: within a scope, use creates shortcuts:
  ```rust
    backyard
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        ├── garden
        │   └── vegetables.rs
        ├── garden.rs
        └── main.rs
    
    // src/main.rs
    use crate::garden::vegetables::Asparagus;
    pub mod garden;
    let asparagus = Asparagus {};

    // src/garden.rs
    pub mod vegetables;

    // src/garden/vegetables.rs
    #[derive(Debug)]
    pub struct Asparagus {}
  ```

With modules, code can be organized for readability/reuse + allows to control privacy of items.

To tell Rust where to find an item in the module tree, **a path is used**:

- absolute: full path starting from a crate root (external crate begins with the crate name, current crate starts with _crate_)
- relative: starts from current module and uses self/super/alias

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

These calls wouldn't compile, because the items aren't public.
To make it work, a path should be exposed.
We need to expose and the container module and the function/s inside it we want to make accessible.


```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

If we use _super_ in a relative path, it's the same as using .. syntax in a filesystem path; it's an easy way to reference something from the parent module.

Structs and enums can be made public with pub. When a struct is defined as pub, we then need to make fields/methods public one by one declaring specifically each of them with pub. Public enums have all its variants public.