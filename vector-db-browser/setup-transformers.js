// setup-transformers.ts
import * as ort from 'onnxruntime-web';
globalThis.ort = ort;

import { env } from '@xenova/transformers';
env.backends.onnx = 'wasm';