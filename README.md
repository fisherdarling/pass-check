# `pass-check`

A tool for exploring compiler optimizations.

## Dependencies

- `LLVM 11`: See https://apt.llvm.org/ for installation instructions
    - From the above link:
    ```bash
    wget https://apt.llvm.org/llvm.sh
    chmod +x llvm.sh
    sudo ./llvm.sh 11
    ```
- `rust`: See https://rustup.rs for installation instructions
- `make`

## Build

Simply run `make` from the root of the project to build `pass-check` and analyze all of the examples in the `examples` directory.

### Showing examples

Each folder inside of `examples/` contains the `opt` and `no_opt` directories (for when compiler optimizations where enabled, and not enabled respectivly).

Inside of each of those folders, there should be an `analysis.json` that describes the instructions in the main function of that example.

Looking at these JSON files should give you an understanding of the compiler was able to optimize instructions, allocations, and function calls.

### Final directory structure of `examples/`
```
.
├── const_folding
│   ├── const_folding.rs
│   ├── no_opt
│   │   ├── analysis.json
│   │   └── const_folding.bc
│   └── opt
│       ├── analysis.json
│       └── const_folding.O.bc
├── loops
│   ├── loops.rs
│   ├── no_opt
│   │   ├── analysis.json
│   │   └── loops.bc
│   └── opt
│       ├── analysis.json
│       └── loops.O.bc
└── sum
    ├── no_opt
    │   ├── analysis.json
    │   └── sum.bc
    ├── opt
    │   ├── analysis.json
    │   └── sum.O.bc
    └── sum.rs

9 directories, 15 files

```

## Usage

To view all of the flags and subcommand, run: `./pass-check help`

```bash
$ ./pass-check help
Fisher D. <fdarling@mines.edu>, Jake V. <jvossen@mines.edu>

USAGE:
    pass-check [FLAGS] [OPTIONS] --target-dir <target-dir> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -j, --json       Output json (used for comparisons)
    -s, --silent     Do not print `Adding Module`
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>            Output to a file
    -t, --target-dir <target-dir>    Target folder that contains the LLVM bitcode (.bc files). Generally under `target/<profile>/deps`

SUBCOMMANDS:
    analyze    Preform analyses
    help       Prints this message or the help of the given subcommand(s)
    search     Search for demangled functions, module names, etc
```

There are two main subcommands in `pass-check`, `search`, and `analyze`.

1. `search`: Use the search subcommand to find a symbol or module name to then further analyze. The analyze subcommand expects demangled symbols, so sure for any function names to get a match.
    ```bash
    $ ./pass-check -t examples/const_folding/opt search func main
    Adding Module const_folding.O.bc examples/const_folding        /opt/const_folding.O.bc

    const_folding::main <-- use this to analyze the main rust function
    main
    ```
2. `analyze`: Analyze a single function or all of the symbols in a module.
    ```bash
    /pass-check -t examples/const_folding/opt analyze func const_folding::main
    Adding Module const_folding.O.bc examples/const_folding/opt/const_folding.O.bc

    FunctionStats {
        name: "const_folding::main",
        instrs: InstructionStats {
            loads: 0,
            stores: 8,
            allocas: 3,
            calls: 7,
            atomic_ops: 0,
            instrs: 30,
        },
        cfg: CFGStats {
            blocks: 1,
            depth: 2,
            branches: 1,
        },
    }
    ```

Finally, the `-j|--json` flag can be used to output data as json.

```bash
./pass-check -j -t examples/const_folding/opt analyze func const_folding::main
    Adding Module const_folding.O.bc examples/const_folding/opt/const_folding.O.bc

{
  "name": "const_folding::main",
  "instrs": {
    "loads": 0,
    "stores": 8,
    "allocas": 3,
    "calls": 7,
    "atomic_ops": 0,
    "instrs": 30
  },
  "cfg": {
    "blocks": 1,
    "depth": 2,
    "branches": 1
  }
}
```

The `-s|--silent` flag can be used to not print the `Adding Module ...` lines.

## Analyze the project itself:

Running this command will give you information about the LLVM bitcode generated from the analizer.

```bash
RUSTFLAGS='--emit llvm-bc' cargo run --release -- -o pass_check_analysis.json -j -t target/release/deps analyze module pass_check
```