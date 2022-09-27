#!/usr/bin/env coffee

import { randomBytes, blake3Round, zipU64, unzipU64, u64Bin, binU64,z85Dump,z85Load } from ".."

loop
  #console.log randomBytes(10)
  li = [2000,50*10000*10000,34359738368]
  x = zipU64 li
  console.log unzipU64 x,li.length

###
  console.log z85Load z85Dump Buffer.from [1,2,3]

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
  console.log await blake3Round [
    new Uint8Array([1]),
    '3wzw23242w'
  ],3
###

