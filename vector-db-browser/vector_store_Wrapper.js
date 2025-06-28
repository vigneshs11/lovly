import { embedText, setEmbedderConfig } from './embedder.js';
import init, { VectorStore } from '../pkg/vector_db_browser.js';
import { set, get, del, keys } from 'https://cdn.skypack.dev/idb-keyval';

export class VectorStoreWrapper {
  constructor(config = {}) {
    setEmbedderConfig(config);
    this.store = new VectorStore();
    this.dbNamespace = config.namespace || 'vector_store';
  }

  async loadFromIndexedDB() {
    const vectorKeys = await keys();
    for (const key of vectorKeys) {
      if (typeof key === 'string' && key.startsWith(this.dbNamespace)) {
        const { id, vector } = await get(key);
        this.store.add_vector(id, new Float32Array(vector));
      }
    }
  }

  async addText(id, text) {
    const vector = await embedText(text);
    this.store.add_vector(id, new Float32Array(vector));
    await set(`${this.dbNamespace}-${id}`, { id, vector });
  }

  async searchText(query, topK = 3) {
    const vector = await embedText(query);
    const results = this.store.search(new Float32Array(vector), topK);
    return JSON.parse(results);
  }

  async deleteVector(id) {
    await del(`${this.dbNamespace}-${id}`);
  }
}
