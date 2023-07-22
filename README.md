# README

Collection of samples to improve mastery of rust programming language & its ecosystem.

## How to start a project

```powershell
$project_name="hello_world"
cargo new $project_name
cd $project_name
```

## Common commands

### Clean

```
cargo clean
```

### Build

Will generate a _dev_ build, available under `.\target\debug`

```
cargo build
```

### Release

Will generate a _prod_ build, available under `.\target\release`

```
cargo build --release
```

### Run

```
cargo run
```

### Check code

```
cargo check
```
