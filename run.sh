#!/usr/bin/env bash

# export TARGET_ARCH=aarch64-unknown-linux-musl
export TARGET_ARCH=aarch64-unknown-linux-gnu
export PKG_CONFIG_FOLDER=aarch64-linux-gnu
# export PKG_CONFIG_PATH=/usr/lib/pkgconfig/:/usr/lib32/pkgconfig/:/usr/lib64/pkgconfig/
export PKG_CONFIG_PATH=/usr/${PKG_CONFIG_FOLDER}/lib/:/usr/${PKG_CONFIG_FOLDER}/lib64/
export PKG_CONFIG_SYSROOT_DIR=/usr/${PKG_CONFIG_FOLDER}/
# export PKG_CONFIG_ALLOW_CROSS=1 


cargo build --target=${TARGET_ARCH} --release -vv
