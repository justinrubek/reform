# Reform

A form building web-app

Currently there is only ui support for creation and use of user accounts

## Build
First, ensure you have a nightly version of rust installed (use [rustup](https://github.com/rust-lang/rustup#installation))

### Frontend
Install [wasm-pack](https://github.com/rustwasm/wasm-pack)

In the `site` directory, use wasm-pack to build the frontend `wasm-pack build --target web`

Then use [rollup](https://rollupjs.org/guide/en/#installation) to create a singular bundle:
`rollup ./main.js --format iife --file ./pkg/bundle.js`

The `pkg` directory will now contain `bundle.js` and `reform_site_bg.wasm`. 
Place these files into the server's static folder under `static/pkg`, renaming the wasm file to `reform_site.wasm`

### Server
Cargo can be used normally to build Rocket:
`cargo build`

## Use

In order for the backend to function, you must create `server/.env` and specify at a minimum the DATABASE_URL.
An example .env file has been included.
When building in debug mode the secret will be a hardcoded string, but be sure to specify SECRET_KEY in the env file for actual use

### Database

This was designed and only tested using postgres.
First, create the database (you can use `server/create_db.sql`)
Use diesel_cli (`cargo install diesel_cli`) to run the migrations for the server: `diesel migration run`
