// Use wit_bindgen to generate the bindings from the component model to Rust.
// For more information see: https://github.com/bytecodealliance/wit-bindgen/
wit_bindgen::generate!({
    path: "..",
    world: "convert",
});

struct GuestComponent;

export!(GuestComponent);

impl Guest for GuestComponent {
    fn convert_celsius_to_fahrenheit(x: f32) -> f32 {
        let result = x * 1.8 + 32.0;
        println!("Result from map module: {} = ", result);
        result
    }
}
