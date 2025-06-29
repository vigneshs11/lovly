// embedder.js
import './setup-transformers';


import { env } from '@xenova/transformers';

// Force the library to use the browser-compatible ONNX backend
env.backends.onnx = 'wasm';

import  {pipeline}  from '@xenova/transformers';
globalThis.ort = ort;

let xenovaEmbedder = null;

export async function embedText(text) {

  console.log('here', xenovaEmbedder)

  
    xenovaEmbedder = await pipeline('feature-extraction', 'Xenova/e5-small', {
      quantized: true,
    });
  

  const output = await xenovaEmbedder(text, {
    pooling: 'mean',
    normalize: true,
  });

  console.log(output.data)
  return output.data;
}
