FROM public.ecr.aws/amazonlinux/amazonlinux:2 as rust_builder_image
WORKDIR /app
ENV PATH=/root/.cargo/bin:/usr/sbin:/usr/bin:/sbin:/bin
ENV BIN=slackwesend

# Setup build environment
RUN yum update -y
RUN yum install -y awscli gcc openssl-devel tree zip unzip tar xz
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
RUN cargo install cargo-chef
RUN curl -Lo /tmp/protobuf.zip https://github.com/protocolbuffers/protobuf/releases/download/v23.1/protoc-23.1-linux-aarch_64.zip
RUN unzip /tmp/protobuf.zip -d /usr

################################################### planner stage (collect dependencies)
FROM rust_builder_image as planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

################################################### cacher stage (build dependencies)
FROM rust_builder_image as cacher
WORKDIR /app
ARG TARGET
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

################################################### builder stage (build binary)
FROM rust_builder_image as builder
WORKDIR /app
COPY --from=cacher /root/.cargo /root/.cargo
COPY --from=cacher /app/target target
COPY . .
RUN cargo build --release
RUN cp target/release/${BIN} bootstrap
RUN zip -9 -j deploy.zip bootstrap

################################################### final stage (copy binary in run time image)
FROM scratch
COPY --from=builder /app/deploy.zip /app/bootstrap /
