import wasmInit, { compile } from "./wasm/megasim_wasm.js";

document.addEventListener('alpine:init', () => {
  Alpine.data('megasim', () => ({
    asmCode: '',
    compiled: '',

    async init() {
      await wasmInit();
    },

    compileAsm() {
      if (!this.asmCode.trim()) return;

      try {
        this.compiled = compile(this.asmCode);
      } catch (e) {
        this.compiled = `Error: ${e.message}`;
      }
    }
  }));
});
