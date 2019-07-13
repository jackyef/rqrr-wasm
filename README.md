# rqrr-wasm

An npm module that use `rqrr` compiled to wasm to use easily in your webpack project.

## Instructions
1. Install the module with its peer dependencies
```
yarn add rqrr-wasm memory-fs fs-extra
```

2. Add the webpack plugin to your webpack configuration
```javascript
import RqrrWasmPlugin from 'rqrr-wasm/dist/webpack-plugins';

// webpack configuration file
webpackConfig = {
  plugins: [
    // other plugins
    new RqrrWasmPlugin(), // put this last
  ],
}
```

3. Import the module in your code
```javascript
import wasmModule from 'rqrr-wasm';
```

4. Call `init()` to load the .wasm file, then you can `decode` a QR code by passing an `Uint8Array` of the image data
```javascript
wasmModule.init().then(() => {
  const canvas = document.createElement('canvas');
  canvas.width = 300;
  canvas.height = 300;

  canvas.getContext('2d').drawImage(video, 0, 0, canvas.width, canvas.height);
  canvas.toBlob(blob => {
    const reader = new FileReader();

    reader.addEventListener('loadend', () => {
      const arrayBuffer = reader.result;
      const output = wasmModule.decode(new Uint8Array(arrayBuffer));

      console.log("DECODED QR: ", output); // the string output here!
    });

    reader.readAsArrayBuffer(blob);
  });
});
```

## Example
An example of how to use this can be seen in this repo https://github.com/jackyef/react-rqrr-wasm
