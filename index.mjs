import { performance } from 'perf_hooks'
import fg from 'fast-glob'
import { extractAllHelpers } from './index.js'
import { __dirname } from './helper.mjs'

const start = performance.now()
const files = fg.sync(['temp/**/*.html'])
const helpers = extractAllHelpers(files)

console.log(helpers, helpers.length)
console.log(performance.now() - start)
