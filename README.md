# Jaust

[![Build Status](https://travis-ci.org/zwennesm/jaust.svg?branch=master)](https://travis-ci.org/zwennesm/jaust)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Jaust stands for **J**ust **A**nother **U**rl **S**hortener **T**ool and the name was inspired by the arcade game [Joust (1982)](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=video&cd=1&cad=rja&uact=8&ved=0ahUKEwiF0-_u3tfiAhUQalAKHRtGDN0QtwIIKjAA&url=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3D2Ga2Dtkg92I&usg=AOvVaw0wPKE7dzKi91GW_zpXy1mE). The project contains a simple web server with a configurable backend (in-memory, Redis) to keep the url shortening persistent (or not).

## How it works

You can either run the application via `docker-compose up` or build your own Docker image. Once the application is running you can access two endpoints.

* `/<id>` which gives a redirect to a given url (if it exists)
* `/console` to shorten a url and get a hashed value

## Roadmap
* Add MySQL support
* Allow configurable back-end (via configuration)
