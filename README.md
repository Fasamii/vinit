# vinit
**Zero-runtime-cost Vulkan initialization with compile-time guarantees.**

`vinit` is a strongly typed Vulkan initialization library built on top of
[`ash`](https://crates.io/crates/ash).  
It uses **type-state**, **generic constraints**, and **compile-time dependency
validation** to ensure that Vulkan objects are created in a correct 
manner before program even runs. Which means that:
- builders that panic at runtime.
- invalid states.
_ half-initialized Vulkan.
*Are simply impossible*.
## ✨ Features
- **Compile-time dependency enforcement**
  - You cannot create a `Device` without an `Instance`
- **Type-state configuration**
  - Missing components are tracked at the type level
- **Zero-cost abstractions**
  - No runtime checks for dependency ordering
- **Modular configuration**
  - Add only what you need (`Instance`, `Device`, command pools, queues)
- **RAII-safe**
  - Vulkan objects are destroyed automatically in the correct order
## Code Example
```rust
use vinit::*;
use std::ffi::CString;

let base = BaseConfig::default()
    .with(
        instance::Instance::default()
            .app_name(CString::new("My App").unwrap())
            .validation(vec![
                CString::new("VK_LAYER_KHRONOS_validation").unwrap(),
            ]),
    )
    .with(device::Device::default())
    .with(command::CommandPool::graphics())
    .with(command::CommandPool::compute())
    .build()
```
# How It Works
vinit uses type-state programming to track which components are present.
At every step:
- Configuration is accumulated in BaseConfig<...>. 
- Each .with(...) consumes the previous BaseConfig and returns a new one
The type system enforces:
- ordering
- dependencies
- required queue families
Which means that code like this
```rust
let base = BaseConfig::default()
    .with(device::Device::default());
```
Wont even compile because device requires Instance to be present
