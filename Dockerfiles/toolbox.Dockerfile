#==============================================================================================
FROM rust:latest as build

WORKDIR /src

COPY . .

RUN cargo clean && \
  cargo build -vv --release --bin toolbox

CMD [ "/src/target/release/toolbox" ]
#==============================================================================================