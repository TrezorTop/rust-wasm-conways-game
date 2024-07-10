#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use web_assembly_binary::Universe;

wasm_bindgen_test_configure!(run_in_browser);

/// Creates a new `Universe` instance with a 6x6 grid and the following alive cells:
/// - (1, 2)
/// - (2, 3)
/// - (3, 1)
/// - (3, 2)
/// - (3, 3)
#[cfg(test)]
pub fn input_universe() -> Universe {
    let mut universe = Universe::new();

    universe.set_width(6);
    universe.set_height(6);

    universe.set_alive_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);

    universe
}

/// Creates a new `Universe` instance with a 6x6 grid and the following alive cells:
/// - (2, 1)
/// - (2, 3)
/// - (3, 2)
/// - (3, 3)
/// - (4, 2)
#[cfg(test)]
pub fn expected_universe() -> Universe {
    let mut universe = Universe::new();

    universe.set_width(6);
    universe.set_height(6);

    universe.set_alive_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);

    universe
}

/// Tests the `tick()` method of the `Universe` struct by creating an input universe,
/// calling `tick()` on it, and asserting that the resulting state matches the expected universe.
#[wasm_bindgen_test]
pub fn test_tick() {
    let mut input_universe = input_universe();
    input_universe.tick();

    let expected_universe = expected_universe();

    assert_eq!(input_universe.get_cells(), expected_universe.get_cells());
}
