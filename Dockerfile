# The goal of this docker image is to provide a container with python3.6 and rust installed
# This is mainly needed as general development is performed on OSX and some tests (travis debug)
# requires linux

# The docker file is not meant to be perfect or the smallest
# In the future the image could be re-customized a bit in order to simplify wheel build -> I'm not at that stage yet

# Example of usage
# $ docker run --rm -it -v $(pwd)/temp-dir:/host-dir --privileged rust-python:latest bash
# # git checkout -f SHA
# # cargo test --no-run
# # kcov /host-dir $(find target/debug/ -maxdepth 1 -name rust_swagger_validator-* -executable)

FROM ubuntu:bionic
MAINTAINER "Samuele Maci <macisamuele@gmail.com>"

RUN set -eux && \
    apt-get update && \
    apt-get install -y \
        binutils-dev \
        cmake \
        cmake \
        curl \
        g++ \
        git \
        jq \
        libcurl4-openssl-dev \
        libdw-dev \
        libelf-dev \
        libiberty-dev \
        libssl-dev \
        pkg-config \
        pkg-config \
        python3-dev \
        python3-pip \
        tig \
        vim \
        zlib1g-dev && \
    rm -rf /var/lib/apt/lists/* && \
    # Install tox
    pip3 install tox && \
    # Install cargo
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly --verbose -y && \
    ${HOME}/.cargo/bin/rustup component add rustfmt-preview --toolchain nightly && \
    ${HOME}/.cargo/bin/rustup component add clippy-preview --toolchain nightly

RUN set -exu && \
    ln /usr/bin/python3 /usr/bin/python && \
    ln /usr/bin/pip3 /usr/bin/pip && \
    # Install cargo-kcov
    ${HOME}/.cargo/bin/cargo install cargo-kcov && \
    ${HOME}/.cargo/bin/cargo kcov --print-install-kcov-sh | sh

ENV PATH=/root/.cargo/bin/:${PATH}


RUN set -eux && \
    git clone --no-checkout --quiet https://github.com/macisamuele/rust-swagger-validator.git /code


WORKDIR /code
CMD ['/bin/bash']
