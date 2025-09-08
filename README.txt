# RightWheel

RightWheel is a Rust application that sets up a low-level mouse hook to intercept mouse wheel events and simulate right-click actions. This tool helps users trigger right-click actions using the mouse wheel.

## Features

- Intercepts mouse wheel events.
- Simulates right-click actions.
- Rate limiting to prevent rapid event firing.
- Automatic cleanup for proper resource management.
- Useful for Minecraft players who want to trigger clicks with the mouse wheel. You can modify the code to send left-click on wheel up and right-click on wheel down, but I recommend using it for right-clicking only for best results.

## Usage

1. Clone the repository:
   ```
   git clone <repository-url>
   cd rightwheel
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the application:
   ```
   cargo run
   ```

4. To terminate the application, press `Ctrl + Shift + Q`. This will completely close the program from Task Manager.

## Dependencies

- `windows`: Windows API bindings.

## User Guide

I am still learning, so I can't provide a `.exe` file on GitHub releases yet. Setting up the Rust environment with C++ development tools might be a bit tricky.

If you want to use this tool, follow these steps:

### Instructions

1. Install Visual Studio 2022.
2. During installation, select "Desktop development with C++".
3. Install Visual Studio Code.
4. In VS Code, install the Rust extension pack.

That's it! This is the easiest way to run the project for now. I plan to release an `.exe` file and improve the code in