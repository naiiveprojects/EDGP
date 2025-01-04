# XDXP

Execute or Debug Godot Engine/GDX Programs.

## Features

### 1. Run Programs

Run your project with the `executable` and `.pck` file located in different directories.

- **Default Directory Behavior**:
  By default, the executable will search for a directory with the same name in the same folder. For example:
  - If the executable is `executable.exe`, it will look for :
    - `executable/`
    - `executable/Bin/win.exe`
    - `executable/Data/main.pck`

  *(Currently, this behavior can only be modified by editing and recompiling it.)*

- **Run With Terminal**:
  To run the program with a terminal, rename the executable with a `.` before the extension (e.g., `executable.debug.exe`). This will execute the program with the `--verbose` and `--debug` flags.

### 2. Run Projects/Editors

Execute the regular project or editor with `--verbose` and `--debug` flags.  

- Place the executable in the same directory as the program you wish to run.  
- Rename the executable to match the program's name with `.` as separator (e.g., `program.test.exe`).  
- This will automatically execute the program with the `--verbose` and `--debug` flags.

## Compiling  

To compile the project, use the following command:  

```bash  
cargo build --release  
```  
