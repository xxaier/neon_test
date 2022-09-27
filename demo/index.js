#!/usr/bin/env -S node --es-module-specifier-resolution=node --trace-uncaught --expose-gc --unhandled-rejections=strict
var n, sleep;

import {
  i64Bin,
  binI64
} from "..";

sleep = () => {
  return new Promise((resolve) => {
    return setTimeout(resolve, 1000);
  });
};

n = 0;

while (true) {
  binI64(i64Bin(1024));
  binI64(i64Bin(256));
  binI64(i64Bin(255));
  binI64(i64Bin(1));
  binI64(i64Bin(-1));
  binI64(i64Bin(-256));
  binI64(i64Bin(-1024));
  if (n % 1000 === 0) {
    await sleep();
    gc();
    console.log('memoryUsage', n, process.memoryUsage());
  }
  n += 1;
}

/*
  console.log z85Load z85Dump Buffer.from [1,2,3]

  console.log i64Bin 1024
  console.log i64Bin 256
  console.log i64Bin 255
  console.log i64Bin 1
  console.log i64Bin -1
  console.log i64Bin -256
  console.log i64Bin -1024

  console.log await blake3Round [
    new Buffer([1]),
    '3wzw23242w'
  ],3
*/
