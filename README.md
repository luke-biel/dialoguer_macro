# Dialoguer trait

It's simple wrapper trait over [dialoguer][dialoguer] crate.
It's target is to remove necessity of providing explicit code when building interactive CLI apps.
It comes with nifty derive macro, that turns (almost) any struct into command prompt configuration.

## Usage

Add dialoguer_macro to your Cargo.toml:

```toml
[dependencies]
dialoguer_macro = "0.1.0" 
```

import trait to your namespace and derive it onto some struct:

```rust
use dialoguer_macro::Dialogue;

#[derive(Dialogue)]
struct Config {
    #[dialogue(prompt = "An option")]
    an_option: String,
    #[dialogue(prompt = "Please input a number")]
    number: isize,
    #[dialogue(prompt = "Are you having a good day?")]
    is_day_good: bool,
}
```

and then all you need to do is compose your struct:

```rust
let config = Config::compose("Give me config... please!").unwrap();
```

## Notes to end user

Project is pretty much WIP.

You have to add `#[dialoguer]` attribute to **EACH** field in your structs, and they must have `prompt` set.  
Deriving `Dialogue` on enum requires that its variants have **EXACTLY ONE UNNAMED** field.  
Support for some types is still missing (eg. PathBuf, UUID, Uri).  
No configuration of CLI that is generated is provided.  
API can change at any point (I'm not fully sure about trait method name).

Help will be much appreciated. 

[dialoguer]: https://github.com/mitsuhiko/dialoguer
