# executable_path_finder

find executable path

```rust
use executable_path_finder::{find, find_with_cargo_home, find_in_env, find_in_path};

fn main() {
    //find in PATH and env vars
    let path = find("cargo");
    println!("{:?}", path);

    //find in PATH, env vars, and cargo home
    let path = find_with_cargo_home("cargo");
    println!("{:?}", path);

    //find in PATH
    let path = find_in_path("cargo");
    println!("{:?}", path);

    //find in env vars
    let path = find_in_env("cargo");
    println!("{:?}", path);
}
```
