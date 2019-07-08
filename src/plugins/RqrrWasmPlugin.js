const fs = require('fs-extra');
const path = require('path');

function RqrrWasmPlugin() {
  const operationName = 'Copying the wasm file to output path...';

  this.apply = function(compiler) {
    compiler.plugin('done', function() {
      /**
       * should return a MemoryFileSystem instance
       * (https://github.com/webpack/memory-fs)
       */
      const webpackOutputPath = compiler.options.output.path;
      const sourcePath = path.resolve(__dirname, '../../dist');
      const fileList = fs.readdirSync(sourcePath);
      const wasmFile = fileList.find(value => /\.wasm$/.test(value));

      const src = path.join(sourcePath, wasmFile);
      const dest =  path.join(webpackOutputPath, wasmFile);

      if (src && dest) {
        try {
          console.log(`[RqrrWasmPlugin] Doing operation: "${operationName}"`);

          const dirname = path.dirname(dest);
          const data = fs.readFileSync(src);

          fs.ensureDirSync(dirname);

          fs.writeFileSync(dest, data);

          console.log(`[RqrrWasmPlugin] Successfully finished operation: "${operationName}"`);
        } catch (err) {
          console.error(`[RqrrWasmPlugin] An error occured while doing operation: "${operationName}"`, err);
          throw new Error(`[RqrrWasmPlugin] An error occured while doing operation: "${operationName}"`);
        }
      }
    });
  }
}

module.exports = RqrrWasmPlugin;