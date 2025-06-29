
console.log('dfd')
import './setup-transformers';

import { env } from '@xenova/transformers';

// Force the library to use the browser-compatible ONNX backend
env.backends.onnx = 'wasm';


import init from './pkg/vector_db_browser.js';
import { VectorStoreWrapper } from './vector_store_wrapper.js';


async function main() {

  await init();
  const store = new VectorStoreWrapper({
    backend: 'xenova',
    modelName: 'Xenova/e5-small',
    namespace: 'vdb_prod_1'
  });

  await store.loadFromIndexedDB();
  await store.addText("vec1", "test");
  await store.addText("vec2", "exam");

  const results = await store.searchText("quiz");
  console.log(results);
}
main();
