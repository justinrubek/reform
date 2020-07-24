# Reform

A form building web-app.
Forms will be designed and then injected into an existing page or even multiple existing pages. 
Filling out the form will drop the data into a database using JSON.

Currently there is only UI support for creation and use of user accounts as well as the start of form building

## Dependencies
* cargo - a nightly version (may be easiest to use [rustup](https://github.com/rust-lang/rustup#installation))
* postgres

## Build

### Frontend
Install [wasm-pack](https://github.com/rustwasm/wasm-pack)

In the `site` directory, use wasm-pack to build the frontend `wasm-pack build --target web`

Then use [rollup](https://rollupjs.org/guide/en/#installation) to create a singular bundle:
`rollup ./main.js --format iife --file ./pkg/bundle.js`

The `pkg` directory will now contain `bundle.js` and `reform_site_bg.wasm`. 
Place these files into the server's static folder under `static/pkg`, renaming the wasm file to `reform_site.wasm`

To ease this process, the site folder contains shell scripts to build the bundle and copy it to the server directory: 
`./build.sh && ./copy.sh`

### Server
Cargo can be used normally to build Rocket:
`cargo build`

## Use

In order for the backend to function, you must create `server/.env` and specify at a minimum the DATABASE_URL.
An example .env file has been included.
When building in debug mode the secret will be a hardcoded string, but be sure to specify SECRET_KEY in the env file for actual use

### Database

First, create the database (you can use `server/create_db.sql`)

Use diesel_cli (`cargo install diesel_cli`) to run the migrations for the server: `diesel migration run`
