use wasm::run;

fn main() {
    pollster::block_on(run());
}
