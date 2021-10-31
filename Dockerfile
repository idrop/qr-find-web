FROM    rust as build
ENV     PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /build
COPY    Cargo.lock Cargo.toml ./
RUN     mkdir src && echo "fn main() {}" > src/main.rs
RUN     cargo install --path . && rm -r src

COPY    src ./src
RUN     cargo build --release

FROM    gcr.io/distroless/cc
WORKDIR /app
COPY    --from=build /build/target/release/app /app
COPY    web /app/web
CMD     ["./app"]
