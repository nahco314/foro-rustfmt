# if ! command -v uv >/dev/null 2>&1; then
#     curl -LsSf https://astral.sh/uv/install.sh | sh
# fi

# uv tool install git+https://github.com/nahco314/dll-pack-builder

echo $(dll-pack-builder find ${BUILD_OUT_DIR})
echo ${DLL_PACK_TARGET} ${GITHUB_REF#refs/tags/}

mkdir ./artifacts/

# cargo build --profile release
LD_LIBRARY_PATH=$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | grep host | awk '{print $2}')/lib \
    dll-pack-builder local $(dll-pack-builder find ${BUILD_OUT_DIR}) ./artifacts/ ${DLL_PACK_TARGET} ${GITHUB_REF#refs/tags/}
