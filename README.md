# foro-rustfmt

This is the rustfmt foro plugin.

**This plugin cannot be used as a WASM plugin.**

rustfmt depends on the rustc library, and can only be built as a target that can build rustc itself. WASM does not fall into this category, so **it can only be used as a native DLL.**

This plugin provides native plug-ins for most platforms, so it usually works fine. for now, x86_64-unknown-linux-gnu, x86_64-apple-darwin, aarch64-apple-darwin and x86_64-pc-windows-msvc are supported.
