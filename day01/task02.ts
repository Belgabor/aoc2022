// deno-lint-ignore-file camelcase ban-unused-ignore
export { }
const test = false
const data = await Deno.readTextFile(test ? './test_data.txt' : './data.txt')
const lines = data.split(/\r?\n/)
console.log(lines)

const elves: number[] = []
let current = 0
let max_calories = 0

function commit() {
    elves.push(current)
    max_calories = Math.max(max_calories, current)
    current = 0
}

for (const line of lines) {
    if (line === '') {
        commit()
        continue
    }
    current += parseInt(line)
}
commit()

const sorted = elves.sort((a, b) => (a<b) ? 1 : (a===b ? 0 : -1))

console.log(sorted)
console.log(sorted[0] + sorted[1] + sorted[2])
