# wasm-cellular-automata

This project is an implementation of Conway's Game of Life using Rust and WebAssembly. The game is rendered on a canvas in the browser.

## Technologies Used

- Rust
- WebAssembly
- JavaScript
- TypeScript
- Vite

## Project Structure

The project is divided into two main parts:

1. The Rust and WebAssembly part, which is responsible for the game logic.
2. The TypeScript part, which is responsible for rendering the game and handling user interactions.

### Rust and WebAssembly

The Rust code is located in the `src` directory. The main logic of the game is implemented in the `Universe` struct in `src/lib.rs`. The `Universe` struct represents the game universe and includes methods for creating a new universe, getting and setting the state of cells, and updating the universe for each tick of the game.

The Rust code is compiled to WebAssembly using the `wasm-pack` tool. The compiled WebAssembly code is located in the `pkg` directory.

### TypeScript

The TypeScript code is located in the `app/src` directory. The `main.ts` file is the entry point of the application. It sets up event listeners for user interactions and starts the game loop. The `universe.ts` file imports the `Universe` class from the compiled WebAssembly code. The `canvas.ts` file handles rendering the game universe on a canvas.

## Running the Project

Compiled .wasm modules are already available in the repository, but you can compile them yourself if you have Rust and `wasm-pack` installed.

To run the project, you must have at least and `npm` installed.

First, compile the Rust code to WebAssembly (you can skip this step if you don't have Rust and `wasm-pack` installed):

```bash
wasm-pack build
```

Then, install the npm dependencies:

```bash
cd app
npm install
```

Finally, start the development server:

```bash
npm run dev
```

The game will be available at `http://localhost:[Port that Vite will choose]`.

## Testing

The project includes tests for the `Universe` struct. The tests are located in the `tests` directory. You can run the tests with the following command:

```bash
wasm-pack test --chrome
```
