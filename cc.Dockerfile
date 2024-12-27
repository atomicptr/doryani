FROM rust:latest

WORKDIR /app

COPY . /app

RUN apt update && \
    apt upgrade -y && \
    apt install -y g++ g++-mingw-w64-x86-64 zip && \
    rustup target add x86_64-unknown-linux-gnu && \
    rustup target add x86_64-pc-windows-gnu && \
    rustup toolchain install stable-x86_64-unknown-linux-gnu && \
    rustup toolchain install stable-x86_64-pc-windows-gnu

RUN cargo build --release --target x86_64-unknown-linux-gnu && \
    cargo build --release --target x86_64-pc-windows-gnu && \
    mkdir /app/builds && \
    tar -czvf /app/builds/doryani-linux-x86_64.tar.gz /app/target/x86_64-unknown-linux-gnu/release/doryani && \
    zip -r /app/builds/doryani-windows-x86_64.zip /app/target/x86_64-pc-windows-gnu/release/doryani.exe && \
    ls -al /app/builds

WORKDIR /build

CMD ["/bin/sh", "-c", "mv /app/builds/doryani-linux-x86_64.tar.gz /build/; mv /app/builds/doryani-windows-x86_64.zip /build/"]
