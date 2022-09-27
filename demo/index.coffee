#!/usr/bin/env coffee

import { i64Bin, binI64 } from ".."

sleep = =>
  new Promise(
    (resolve) =>
      setTimeout(resolve, 1)
  )

n = 0
loop
  binI64 i64Bin 1024
  binI64 i64Bin 256
  binI64 i64Bin 255
  binI64 i64Bin 1
  binI64 i64Bin -1
  binI64 i64Bin -256
  binI64 i64Bin -1024

  await sleep()
  gc()
  if n%1000 == 0
    console.log 'memoryUsage', n, process.memoryUsage()
  n += 1


###
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
###

