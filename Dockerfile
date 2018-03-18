FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/app
RUN USER=root cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY src templates ./
RUN cargo build --release

FROM node:9 as builder-web
WORKDIR /usr/src/app
COPY yarn.lock package.json webpack.config.js .babelrc ./
RUN yarn
COPY app /usr/src/app/app
RUN yarn build

FROM scratch
COPY --from=builder /usr/src/app/target/release/good_stv_server /bin/
COPY --from=builder-web /usr/src/app/public public
ENTRYPOINT good_stv_server
