// deno-lint-ignore-file camelcase ban-unused-ignore
export { }
const test = false
const data = await Deno.readTextFile(new URL(test ? 'test_data.txt' : 'data.txt', import.meta.url))
const lines = data.split(/\r?\n/)
console.log(lines)

interface Instruction {
    count: number
    from: number
    to: number
}

const instructions: Instruction[] = []
const stacks: string[][] = []


function parse() {
    const map_lines: string[] = []
    let map_finished = false
    for (const line of lines) {
        if (map_finished) {
            const parts = line.split(' ')
            instructions.push({
                count: parseInt(parts[1]),
                from: parseInt(parts[3]),
                to: parseInt(parts[5]),
            })
        } else {
            if (line === '') {
                map_finished = true
                continue
            }

            map_lines.push(line)
        }
    }
    const map_id = map_lines.pop()!.trim()
    const stack_count = map_id.split('   ').length
    for (let i = 0; i < stack_count; i++) {
        stacks.push([])
    }
    for (const map_line of map_lines.toReversed()) {
        for (let i = 0; i < stack_count; i++) {
            const index = 1 + 4*i
            const box = map_line.substring(index, index+1)
            if (box !== ' ') {
                stacks[i].push(box)
            }
        }
    }
}

function move(instruction: Instruction) {
    const from_stack = stacks[instruction.from - 1]
    stacks[instruction.from - 1] = from_stack.slice(0, -instruction.count)
    stacks[instruction.to - 1].push(...from_stack.slice(-instruction.count)/* .toReversed() */)
}

function result() {
    let res = ''
    for (const stack of stacks) {
        res += stack.pop()
    }
    return res
}

parse()
console.log(stacks)
for (const instruction of instructions) {
    move(instruction)
}
console.log(stacks)
console.log(result())

//console.log(instructions)
