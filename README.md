# Jaust

[![Build Status](https://travis-ci.org/zwennesm/jaust.svg?branch=master)](https://travis-ci.org/zwennesm/jaust)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Jaust stands for **J**ust **A**nother **U**rl **S**hortener **T**ool and the name was inspired by the arcade game [Joust (1982)](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=video&cd=1&cad=rja&uact=8&ved=0ahUKEwiF0-_u3tfiAhUQalAKHRtGDN0QtwIIKjAA&url=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3D2Ga2Dtkg92I&usg=AOvVaw0wPKE7dzKi91GW_zpXy1mE). The project contains a simple web server with a configurable backend (in-memory, Redis) to keep the url shortening persistent (or not).

## How it works

Jaust consists of two separate parts:
* An endpoint which can be used with a shortened id to get redirected.
* An admin console where new shortened ids can be added.

This translates to the following endpoints:
* `/<id>` which gives a redirect to a given url (if it exists)
* `/console` to shorten a url and get a hashed value

The application can be run in two modes: **with** or **without** the admin console. This path is chosen
out of security reasons. Enabling or disabling the admin console is done with an environment value:

```CONSOLE_ENABLED=true/false```

![screenshot of the admin console](docs/screenshot.png)

## Run it

The application can be run directly with cargo by using `cargo run`. However keep in mind
that a `DATABASE_URL` environment variable needs to be supplied. This can be done by adding an entry
to your `.env` file or by supplying it to the cargo command like this:

```
DATABASE_URL=postgres://postgres:password@localhost/jaust cargo run
```

## Development

The first time you deploy this application locally you need to set up the database first. This can be done with the
following commands.

Run a local PostgreSQL database:

```
docker run --name postgres -p 5432:5432 -e POSTGRES_PASSWORD=password -e POSTGRES_DB=jaust -d postgres
```

Install the diesel cli (the ORM that is used):

```
cargo install diesel_cli --no-default-features --features postgres
```

Run the database migrations:

```
DATABASE_URL=postgres://postgres:password@localhost/jaust 
diesel setup
diesel migration run
```

## Build it

You can either run the application via `docker-compose up` or build your own Docker image. The repository 
already contains a `Dockerfile`. Noteworthy: the image is built using `musl` (https://musl.libc.org/) in order 
to produce a very small docker image.  

## Roadmap
* Add MySQL support next to PostgreSQL support
* Allow configurable back-end (via configuration)
