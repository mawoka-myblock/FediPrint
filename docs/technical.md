# Technical docs

## Basics

FediPrint is made up of three major components, the "app", the "worker"(s) and the "frontend".
The app and worker are connected via Postgres, the central (point of failure) for FediPrint.
The app and the worker share the same code for db communication and some other small helpers.
The whole db interface is written using pure SQL with Sqlx as a helper and compile-time checker.

## App

The app is written in Rust and uses the Axum framework. The state is passed around the whole program,
which includes the db pool, the stripe client, the current user, the env config and more. There should be **no SQL** in there,
all should be located in the `shared` model. This is necessary, as changing the old SQL queries will not work
and will have to be updated and having that more or less central is nice.


## Worker
This part runs background processes. In theory, you can throw as many workers at it as you want, as the task queue is handled by Postgres.
This process uses notifications and locks (<3 Postgres) and can handle recurring tasks, retries and more.

## Shared
This cargo workspace is where all the database code lives. There's also the env-config and many other smaller helper functions.

## Frontend
The frontend is built with Svelte 5 (alpha) as it'll be released shortly and introduced runes, which are enabled in almost any component!
For styling, it uses UnoCSS (basically a better TailwindCSS). The JWT is read and available in a store on server and client components.
