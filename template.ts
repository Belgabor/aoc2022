// deno-lint-ignore-file camelcase ban-unused-ignore
export { }
const test = true
const data = await Deno.readTextFile(new URL(test ? 'test_data.txt' : 'data.txt', import.meta.url))
const lines = data.split(/\r?\n/)
console.log(lines)

