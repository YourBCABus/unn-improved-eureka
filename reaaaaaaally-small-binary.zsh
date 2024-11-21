#!/bin/zsh

if [ -z "${CARGO_TARGET_DIR}" ]; then 
    TARGET='target'
else 
    TARGET=${CARGO_TARGET_DIR}
fi

cargo build --profile release-minimum --target x86_64-unknown-linux-gnu || true

mkdir $TARGET/tiny-eureka
cp $TARGET/x86_64-unknown-linux-gnu/release-minimum/improved-eureka $TARGET/tiny-eureka/improved-eureka

upx -9 $TARGET/tiny-eureka/improved-eureka

du -Ah $TARGET/tiny-eureka/improved-eureka
