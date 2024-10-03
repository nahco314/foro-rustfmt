# foro-rustfmt

This is the rustfmt foro plugin.

**This plugin cannot be used as a WASM plugin.**

rustfmt depends on the rustc library, and can only be built as a target that can build rustc itself. WASM does not fall into this category, so **it can only be used as a native DLL.**

Also, the rustc library is linked as a dynamic library (and the version of the toolchain used to build rustfmt is different from the one you normally use), so you need to manually set the RPATH for the foro-rustfmt library (for some reason, cargo's automatic specification of RPATH doesn't work).

## Workaround

プラグインを使用せずに

## Install / Usage

You need to build this project manually, set the RPATH, and set the path in the foro.json.

### on linux

`patchelf` is required:

```shell
sudo apt install patchelf
```

Build and set RPATH:

```shell
git clone https://github.com/nahco314/foro-rustfmt
cd foro-rustfmt

cargo build --profile super-release
patchelf --set-rpath $(rustc --print sysroot)/lib/rustlib/$(rustc -vV | grep host | awk '{print $2}')/lib ./target/super-release/libforo_rustfmt.so
```

<details>
<summary>If you use fish shell</summary>

```shell
git clone https://github.com/nahco314/foro-rustfmt
cd foro-rustfmt

cargo build --profile super-release
patchelf --set-rpath (rustc --print sysroot)/lib/rustlib/(rustc -vV | grep host | awk '{print $2}')/lib ./target/super-release/libforo_rustfmt.so
```
</details>

And add the following rule to your `foro.json`:

```json
{
  "rules": [
    {
      "on": ".rs",
      "cmd": {
        "__deprecation_native_dll": "/path/to/foro-rustfmt/target/super-release/libforo_rustfmt.so"
      }
    }
  ]
}
```

### on mac

Instructions for Mac are not currently available. If you change the 'patchelf' part of the linux command appropriately, it should probably work (contributions wanted!).

### on windows

Windows has not been tested at all. It is probably the same situation as Linux, but I don't know if the same method using RPATH will solve the problem.
