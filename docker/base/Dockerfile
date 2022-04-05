FROM rust:1.58.1

RUN curl -OL https://github.com/Kitware/CMake/releases/download/v3.23.0-rc3/cmake-3.23.0-rc3.tar.gz
RUN tar xf cmake-3.23.0-rc3.tar.gz
WORKDIR ./cmake-3.23.0-rc3
RUN ./configure
RUN gmake
RUN gmake install
WORKDIR /
RUN apt update
RUN apt -y install libclang-dev
