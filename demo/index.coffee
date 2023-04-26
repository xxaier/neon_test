#!/usr/bin/env coffee

import * as rust from "../index.js"

{ i64Bin, binI64 } = rust

sleep = =>
  new Promise(
    (resolve) =>
      setTimeout(resolve, 1000)
  )

{rss} = process.memoryUsage()

n = 0
loop
  binI64 i64Bin 1024
  binI64 i64Bin 256
  binI64 i64Bin 255
  binI64 i64Bin 1
  binI64 i64Bin -1
  binI64 i64Bin -256
  binI64 i64Bin -1024

  if n%1000 == 0
    await sleep()
    gc()
    leak = parseInt((process.memoryUsage().rss-rss)/1024/1024)

    console.log 'loop', n, 'leak', leak+' MB'
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

