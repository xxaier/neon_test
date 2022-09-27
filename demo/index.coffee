#!/usr/bin/env coffee

import { blake3Round, zipU64, unzipU64, u64Bin, binU64,z85Dump,z85Load } from ".."

li = [2000,50*10000*10000,34359738368]
x = zipU64 li
console.log unzipU64 x,li.length
#console.log new Buffer(x).readUIntLE(0,x.length)

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
###

begin = new Date()
console.log await blake3Round [
  new Uint8Array([1]),
  '3wzw23242w'
],3e3
console.log (new Date()-begin)/1000
