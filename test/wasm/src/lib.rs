use std::ops::Range;

use wasmer::{Array, Instance, Memory, Module, NativeFunc, Store, WasmPtr};
use wasmer_wasi::WasiState;

use std::mem;


pub struct Judge {
    instance: Instance,
    generate: NativeFunc<(), (u32, WasmPtr<u8, Array>)>,
    check: NativeFunc<WasmPtr<u8, Array>, u8>,
    allocate: NativeFunc<(u32, u32), WasmPtr<u8, Array>>,
    deallocate: NativeFunc<(u32, u32, u32), WasmPtr<u8, Array>>,
}

impl Judge {
    pub fn new(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let store = Store::default();
        let module = Module::new(&store, bytes)?;

        // Create the `WasiEnv`.
        let mut wasi_env = WasiState::new("judge").finalize()?;

        // Generate an `ImportObject`.
        let import_object = wasi_env.import_object(&module)?;

        // Let's instantiate the module with the imports.
        let instance = Instance::new(&module, &import_object)?;

        // Let's call the `_start` function, which is our `main` function in Rust.
        // let start = instance.exports.get_function("_start")?;
        // start.call(&[])?;

        let generate = instance.exports.get_native_function("_generate")?;
        let check = instance.exports.get_native_function("_check")?;
        let allocate = instance.exports.get_native_function("_allocate")?;
        let deallocate = instance.exports.get_native_function("_deallocate")?;

        Ok(Self {
            instance,
            generate,
            check,
            allocate,
            deallocate
        })
    }
    
    pub fn generate(&self) -> Result<String, Box<dyn std::error::Error>> {
        let memory = self.instance.exports.get_memory("memory")?;

        let (length, input) = self.generate.call()?;

        Ok(input.get_utf8_string(memory, length).unwrap_or_default())
    }

    pub fn check(&self, output: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.check();

        let ptr = self.allocate(size, allign);

        self.check.call(output.len(), ptr);

        self.deallocate(ptr, size, allign);

        todo!()
    }
}
