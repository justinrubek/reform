import init, { run_app } from './pkg/reform_site.js';
async function main() {
   await init('/pkg/reform_site.wasm');
   run_app();
}
main()
