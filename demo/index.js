#!/usr/bin/env -S node --es-module-specifier-resolution=node --trace-uncaught --expose-gc --unhandled-rejections=strict
var n;

import {
  u64Bin,
  binU64
} from "..";

n = 0;

while (true) {
  binU64(u64Bin(1024));
  binU64(u64Bin(256));
  binU64(u64Bin(255));
  binU64(u64Bin(1));
  binU64(u64Bin(-1));
  binU64(u64Bin(-256));
  binU64(u64Bin(-1024));
  gc();
  if (n % 1000 === 0) {
    console.log('memoryUsage', n, process.memoryUsage());
  }
  n += 1;
}

/*
  console.log z85Load z85Dump Buffer.from [1,2,3]

  console.log u64Bin 1024
  console.log u64Bin 256
  console.log u64Bin 255
  console.log u64Bin 1
  console.log u64Bin -1
  console.log u64Bin -256
  console.log u64Bin -1024

  console.log await blake3Round [
    new Buffer([1]),
    '3wzw23242w'
  ],3
*/
