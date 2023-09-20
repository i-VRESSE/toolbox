#==============================================================================================
FROM rust:latest as build

WORKDIR /src

COPY toolbox .

RUN cargo clean && \
  cargo build -vv --release

EXPOSE 8080
CMD [ "/src/target/release/toolbox" ]
#==============================================================================================