const bs58 = require('bs58');
const fs = require('fs');
const privateKeyString = 'FDYxSoWcZSPJhcm6cjmd1bNCVT4jNB5foohjz1d22KwG';
const b = bs58.decode(privateKeyString);
const j = new Uint8Array(b.buffer, b.byteOffset, b.byteLength / Uint8Array.BYTES_PER_ELEMENT);
fs.writeFileSync('key.json', `[${j}]`);