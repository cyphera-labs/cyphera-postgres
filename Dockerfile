FROM postgres:16

RUN apt-get update && apt-get install -y \
    build-essential \
    postgresql-server-dev-16 \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /usr/local/src/cyphera
COPY cyphera_udf.c Makefile cyphera--1.0.sql cyphera.control /usr/local/src/cyphera/

WORKDIR /usr/local/src/cyphera
RUN make && make install

COPY init.sql /docker-entrypoint-initdb.d/01-cyphera.sql
