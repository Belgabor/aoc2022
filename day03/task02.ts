// deno-lint-ignore-file camelcase ban-unused-ignore
export { }
const test = false
const data = await Deno.readTextFile(test ? './test_data.txt' : './data.txt')
const lines = data.split(/\r?\n/)
console.log(lines)

let score = 0

const a = 'a'.charCodeAt(0)
const A = 'A'.charCodeAt(0)

function get_distribution_map(compartment: string) {
    const map: Record<string, number> = {}

    for (const letter of compartment) {
        map[letter] = (map[letter] ?? 0) + 1
    }

    return map
}

function evaluate(letter: string) {
    const code = letter.charCodeAt(0)
    if (code < a) {
        return 27 + (code - A)
    }
    return 1 + (code - a)
}

let members: string[] = []

function commit() {
    const first = members[0]
    const second = members[1]
    const third = members[2]

    const first_map = get_distribution_map(first)
    const second_map = get_distribution_map(second)
    const third_map = get_distribution_map(third)

    let letter: string|undefined = undefined
    for (const first_letter of Object.keys(first_map)) {
        if (second_map[first_letter] !== undefined && third_map[first_letter] !== undefined) {
            letter = first_letter
            break
        }
    }

    if (letter === undefined) {
        throw new Error('Not found')
    }
    const priority = evaluate(letter)
    //console.log(letter, priority)
    score += priority

    members = []
}

for (const line of lines) {
    members.push(line)
    if (members.length === 3) {
        commit()
    }
}

console.log(score)
