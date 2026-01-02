import wasmInit, { Simulator } from "./wasm/megasim_wasm.js";

document.addEventListener('alpine:init', () => {
    Alpine.data('megasim', () => ({
        asmCode: '',
        compiled: '',
        sim: null,
        pc: 0,
        ram: new Uint8Array(1120),
        isReady: false,
        
        isRunning: false,
        renderReqId: null,

        async init() {
            try {
                await wasmInit();
                this.isReady = true;
                console.log("WASM Loaded.");
            } catch (e) {
                console.error("WASM Load Failed:", e);
                this.compiled = "Error loading WebAssembly. Check console.";
            }
        },

        compileAsm() {
            if (!this.isReady) return;
            if (!this.asmCode.trim()) return;

            this.stopRun();

            try {
                if (this.sim) {
                    this.sim.free();
                }

                this.sim = new Simulator(this.asmCode);
                this.compiled = this.sim.program_str();
                this.updateState();

            } catch (e) {
                console.error("Compilation Error:", e);
                this.compiled = `Error: ${e}`;
                this.sim = null;
            }
        },

        toggleRun() {
            if (this.isRunning) {
                this.stopRun();
            } else {
                this.startRun();
            }
        },

        startRun() {
            if (!this.sim) return;
            this.isRunning = true;

            this.runSimLoop();
            this.runRenderLoop();
        },

        stopRun() {
            this.isRunning = false;
            if (this.renderReqId) {
                cancelAnimationFrame(this.renderReqId);
                this.renderReqId = null;
            }
        },

        runSimLoop() {
            if (!this.isRunning) return;

            try {
                for (let i = 0; i < 5_000; i++) {
                    const keepGoing = this.sim.step();
                    if (!keepGoing) {
                        this.stopRun();
                        return;
                    }
                }
            } catch (e) {
                this.handleCrash(e);
                return;
            }

            this.updateState();
            setTimeout(() => this.runSimLoop(), 0);
        },

        runRenderLoop() {
            if (!this.isRunning) return;

            this.renderReqId = requestAnimationFrame(() => this.runRenderLoop());
        },

        manualStep() {
          this.step();
          this.updateState();
        },

        step() {
            if (!this.sim || this.isRunning) return;

            try {
                const keepGoing = this.sim.step();
                if (!keepGoing) console.log("CPU Halted.");
            } catch (e) {
                this.handleCrash(e);
            }
        },

        handleCrash(e) {
            console.error("Runtime Panic:", e);
            this.compiled = `RUNTIME ERROR!\n${e}`;
            this.stopRun();
            
            if (this.sim) {
                this.sim.free();
                this.sim = null;
            }
        },

        updateState() {
            if (!this.sim) return;
            const state = this.sim.state();
            this.pc = state.pc;
            this.ram = state.ram;
        },

        get registers() {
            return Array.from(this.ram.subarray(0, 32)); 
        },

        get ioRegs() {
            return Array.from(this.ram.subarray(32, 96));
        },

        isBitOn(byteVal, bitIdx) {
            return (byteVal & (1 << (7 - bitIdx))) !== 0;
        },

        toggleBit(byteIdx, bitIdx) {
            this.sim.set_byte(byteIdx, this.ram[byteIdx] ^ (1 << (7 - bitIdx)));
            this.updateState();
        }
    }));
});