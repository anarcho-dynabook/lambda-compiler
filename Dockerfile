FROM ubuntu:24.04

RUN apt-get update
RUN apt-get install -y nasm gcc build-essential curl

ENV RUSTUP_INIT_ARGS="-y"
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR ../lambda-compiler

CMD ["bash"]
