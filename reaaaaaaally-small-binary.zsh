#!/bin/zsh

cargo build --profile release-minimum --target x86_64-unknown-linux-gnu || true

mkdir ./target/tiny-eureka
cp ./target/x86_64-unknown-linux-gnu/release-minimum/improved-eureka ./target/tiny-eureka/improved-eureka

upx -9 ./target/tiny-eureka/improved-eureka

du -Ah ./target/tiny-eureka/improved-eureka
