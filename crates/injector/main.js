import init, { inject_forms } from './pkg/reform_injector.js';
async function main() {
   await init('/pkg/reform_injector.wasm');
   inject_forms();
}
main()
