FROM public.ecr.aws/amazonlinux/amazonlinux:2023 as rust_builder_image
ARG BIN

WORKDIR /app
ENV PATH=/root/.cargo/bin:/usr/sbin:/usr/bin:/sbin:/bin
ENV BIN=${BIN}


# Setup build environment
RUN yum update -y
RUN yum install -y awscli gcc openssl-devel tree zip unzip tar xz
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
RUN cargo install cargo-chef
RUN curl -Lo /tmp/protobuf.zip https://github.com/protocolbuffers/protobuf/releases/download/v23.1/protoc-23.1-linux-aarch_64.zip
RUN unzip /tmp/protobuf.zip -d /usr

################################################### planner stage (collect dependencies)
FROM rust_builder_image as planner
ARG BIN
WORKDIR /app
COPY . .
RUN cargo chef prepare --bin=${BIN} --recipe-path recipe.json

################################################### cacher stage (build dependencies)
FROM rust_builder_image as cacher
ARG BIN
WORKDIR /app
ARG TARGET
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --bin=${BIN} --release --recipe-path recipe.json

################################################### builder stage (build binary)
FROM rust_builder_image as builder
ARG BIN
WORKDIR /app
COPY --from=cacher /root/.cargo /root/.cargo
COPY --from=cacher /app/target target
COPY . .
RUN cargo build --bin=${BIN} --release
RUN mkdir out
RUN cp target/release/${BIN} out/bootstrap
RUN strip out/bootstrap
RUN find out
RUN cd out && zip -9 -r deploy.zip .

################################################### final stage (copy binary in run time image)
FROM scratch
COPY --from=builder /app/out/deploy.zip /
