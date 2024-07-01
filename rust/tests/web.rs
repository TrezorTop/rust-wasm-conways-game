#![cfg(target_arch = "wasm32")]


use wasm_bindgen_test::*;
use web_assembly_binary::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_universe() -> Universe {
    let mut universe = Universe::new();

    universe.set_width(6);
    universe.set_height(6);

    universe.set_alive_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);

    universe
}

#[cfg(test)]
pub fn expected_universe() -> Universe {
    let mut universe = Universe::new();

    universe.set_width(6);
    universe.set_height(6);

    universe.set_alive_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);

    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut input_universe = input_universe();
    input_universe.tick();

    let expected_universe = expected_universe();

    assert_eq!(input_universe.get_cells(), expected_universe.get_cells());
}
