#!/usr/bin/env coffee

import { u64Bin, binU64 } from ".."

loop
  console.log u64Bin 1024
  console.log u64Bin 256
  console.log u64Bin 255
  console.log u64Bin 1
  console.log u64Bin -1
  console.log u64Bin -256
  console.log u64Bin -1024
  console.log binU64 u64Bin 1024
  console.log binU64 u64Bin 256
  console.log binU64 u64Bin 255
  console.log binU64 u64Bin 1
  console.log binU64 u64Bin -1
  console.log binU64 u64Bin -256
  console.log binU64 u64Bin -1024

###
  console.log z85Load z85Dump Buffer.from [1,2,3]

  console.log u64Bin 1024
  console.log u64Bin 256
  console.log u64Bin 255
  console.log u64Bin 1
  console.log u64Bin -1
  console.log u64Bin -256
  console.log u64Bin -1024

  console.log await blake3Round [
    new Uint8Array([1]),
    '3wzw23242w'
  ],3
###

