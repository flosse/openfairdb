# Open Fair DB

The backend for [Karte von morgen](https://github.com/kartevonmorgen/kartevonmorgen/)
written in [Rust](http://rustlang.org/).

[![Build Status](https://travis-ci.org/kartevonmorgen/openfairdb.svg?branch=master)](https://travis-ci.org/kartevonmorgen/openfairdb)
[![Coverage Status](https://coveralls.io/repos/github/kartevonmorgen/openfairdb/badge.svg?branch=master)](https://coveralls.io/github/kartevonmorgen/openfairdb?branch=master)
[![dependency status](https://deps.rs/repo/github/kartevonmorgen/openfairdb/status.svg)](https://deps.rs/repo/github/kartevonmorgen/openfairdb)
[![License](https://img.shields.io/badge/license-AGPLv3-blue.svg?style=flat)](https://github.com/kartevonmorgen/openfairdb/blob/master/LICENSE)

## REST API

The API is available under `http://api.ofdb.io/v0/`.

The current API is documented within the [openapi.yaml](https://github.com/kartevonmorgen/openfairdb/blob/master/openapi.yaml) file.
You can render the API documentation e.g. with the swagger editor:

- go to [https://editor.swagger.io](https://editor.swagger.io/)
- go to `File` -> `import URL`
- enter `https://raw.githubusercontent.com/kartevonmorgen/openfairdb/master/openapi.yaml`
- enjoy ;-)

An other way to see how the API can be used, you can open the `network` tab in the developer
tools of your browser and see the requests that are made to `https://kartevonmorgen.org`.

If you want to use the API in your project, please contact us at helmut@bildungsagenten.com.
The API might still change sometimes. We will try to let you know in that case.

-  `GET /entries/:ID_1,:ID_2,...,:ID_n`
-  `PUT /entries/:ID`
-  `GET /categories/`
-  `GET /categories/:ID_1,:ID_2,...,:ID_n`
-  `POST /ratings`
-  `GET /ratings`
-  `GET /ratings/:ID_1,:ID_2,...,:ID_n`
-  `POST /users`
-  `POST /confirm-email-address`
-  `GET /bbox-subscriptions`
-  `POST /subscribe-to-bbox`
-  `POST /unsubscribe-all-bboxes`

### Login & Subscriptions

For the following requests one must be logged in:
`GET /users/:USERNAME`
`GET /bbox-subscriptions`
`POST /subscribe-to-bbox`
`POST /unsubscribe-all-bboxes`

`bbox-subscriptions` are subscriptions to a certain map area (bounding box,`bbox`): whenever a new entry is created or an entry is changed within that area, an email notification is sent to the user.

## Quick start

Download the latest build
[openfairdb_x86_64-unknown-linux-musl_v0.5.1.tar.xz](https://github.com/kartevonmorgen/openfairdb/releases/download/v0.5.1/openfairdb_x86_64-unknown-linux-musl_v0.5.1.tar.xz),
unpack and start it:

```sh
wget https://github.com/kartevonmorgen/openfairdb/releases/download/v0.5.1/openfairdb_x86_64-unknown-linux-musl_v0.5.1.tar.xz
tar xJf openfairdb_x86_64-unknown-linux-musl_v0.5.1.tar.xz
RUST_LOG=info ./openfairdb
```

## Build

Requirements:

- [Rust](https://www.rust-lang.org/) (nightly)
- [SQLite](https://sqlite.org/) 3.x

### Installing Rust & Cargo

If you're using Ubuntu 16.04 LTS you can run

```sh
sudo apt-get install curl libssl-dev gcc
curl https://sh.rustup.rs -sSf | sh
```

On Windows you can download the installer from [rustup.rs](https://rustup.rs).
(But don't forget to install a
[C++ toolchain](http://landinghub.visualstudio.com/visual-cpp-build-tools) first).

### Installing SQLite & Diesel

On Ubuntu:

```sh
sudo apt-get install sqlite3 libsqlite3-dev
cargo install diesel_cli --no-default-features --features sqlite
```

### Compile & Run

```sh
git clone https://github.com/kartevonmorgen/openfairdb
cd openfairdb/
diesel migration run
cargo build
./target/debug/openfairdb
```

The required Rust toolchain and version is defined in *rustc-toolchain* and
will be installed by *Cargo* on demand when building the project.

On NixOS you can build the project with:

```sh
nix-build -E '(import <nixpkgs>{}).callPackage ./default.nix {}'
```

## Logging

```sh
RUST_LOG=debug ./target/debug/openfairdb
```

If you want to get stacktraces on panics use

```sh
export RUST_BACKTRACE=1
```

### Docker

#### Build the image

Build and tag the Docker image:

```sh
docker build -t openfairdb:latest .
```

The image is created `FROM scratch` and does not provide any user environment or shell.

#### Run the container

The executable in the container is controlled by the following environment variables:

- RUST_LOG: Log level (trace, debug, info, warn, error)
- DATABASE_URL: Database file path

The database file must be placed in a volume outside of the container. For
this purpose the image defines the mountpoint */volume* where an external volume
from the host can be mounted.

The container exposes the port 8080 for publishing to the host.

Example:

```sh
docker run --rm \
    -p 6767:8080 \
    -e RUST_LOG="info" \
    -e DATABASE_URL="/volume/openfairdb.sqlite" \
    -v "/var/openfairdb":/volume:Z \
    openfairdb:latest
```

#### Extract the static executable

The resulting Docker image contains a static executable that can be extracted from any container instance:

```sh
docker cp <container id>:openfairdb .
```

## DB Backups

At the moment the OpenFairDB does not support online backups.
Therefore we use a simple
[script](https://github.com/kartevonmorgen/openfairdb/blob/master/scripts/backup-sqlite.sh)
that copies the DB file once a day.

# Domain Model

*![The rendered class diagram should appear here!](http://www.plantuml.com/plantuml/svg/RLJ1Yjim4BtxAqIEWLtQQp1XswM7maAXsvx3n1uKiVQCaSRj9gN_NbLZErQK76BhlQStencDduA0bx7lgghf80JpgMqznkUVoiHVu-IyCw_Y7La5U2JnEHR48qe6NTomhF_Erf-F_5vL___Dzk5XRpQ1HpaTVcCGyt5ZdfbzwmW4rnfY7pK8XMPb-ZeUG-FT88x9r3MInBJt-wegoCrsOv9jzFePq9kT2SeVCHXXKvTxjlC6pL_3FeEWPN_EmaqKztt4CcR6eiqI_pk88nipQ9GCPcL10erCJS0UN9ULzyGz3c0n0mKx74vCM5R-MhR9iWFPcHSG9sEBYf2D29DLQDdwXIGxvMpW6gIG9-1wi7WOVNS7xHozPLGCeDRQalHOYXfheg_kWi7KfV87s2WIi0kxj6aktYtymj7JCIq7-tNRf8H4RN556eyWceXAxYUYR9b83XU9NDVpswJzyFWOvTD0tf831vUMTwVYcxT0xg8RYkR1u0x2RqZhRcHRYXFstA87mTKbrVjRkZTCWk_vzy0dxSvyZPH5dx30es-mk13tPqHZrqjixZ157ljby5AcnJXg3wzmELCQEydc7YN_gdf2QiU--mS0)*

Note: Currently the rendered class diagram must be updated manually by uploading the contents of the file [classes.puml](classes.puml) to the [PlantUML Online Editor](http://www.plantuml.com/plantuml/uml/) and replace the link for the rendered diagram with one of the generated URLs.

# License

Copyright (c) 2015 - 2018 Markus Kohlhase

This project is licensed under the AGPLv3 license.
