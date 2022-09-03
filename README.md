# rust-fullstack-example

An example of creating a full stack web application (backend and frontend) using Rust.

## Prerequisites

Install [Trunk](https://trunkrs.dev/#install).

## Database (Docker)

To run [PostgreSQL](https://www.postgresql.org/) using [docker-compose](https://hub.docker.com/_/postgres), run:

- `docker-compose -f docker-compose.yml up`, press `Ctrl + C` to quit
- `docker-compose -f docker-compose.yml up -d` to run in detached mode, and
  run `docker-compose -f docker-compose.yml down` when done

## Backend

Go to `./backend` and start the server using `make dev`.

## Frontend

Go to `./frontend` and start a local server on port 8080 using `make web`.

You can visit the frontend by going to http://127.0.0.1:8080.

You can create owners and for each owner, on their detail page, create pets and delete them.
