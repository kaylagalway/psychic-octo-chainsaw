# syntax=docker/dockerfile:1.4
FROM rust:buster AS base

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code