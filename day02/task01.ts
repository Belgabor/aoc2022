// deno-lint-ignore-file camelcase ban-unused-ignore
export { }
const test = false
const data = await Deno.readTextFile(test ? './test_data.txt' : './data.txt')
const lines = data.split(/\r?\n/)
console.log(lines)

enum Sign {
    Rock,
    Paper,
    Scissors,
}

const map = {
    X: Sign.Rock,
    Y: Sign.Paper,
    Z: Sign.Scissors,
    A: Sign.Rock,
    B: Sign.Paper,
    C: Sign.Scissors,
}

const sign_score = {
    [Sign.Rock]: 1,
    [Sign.Paper]: 2,
    [Sign.Scissors]: 3,
}

type Code = keyof typeof map

let score = 0

function evaluate(opponent: Sign, me: Sign) {
    let score = sign_score[me]

    if (opponent === me) {
        score += 3
    } else if (
        (opponent === Sign.Rock && me === Sign.Paper)
         || (opponent === Sign.Paper && me === Sign.Scissors)
         || (opponent === Sign.Scissors && me === Sign.Rock)
         ) {
        score += 6
    }

    return score
}

for (const line of lines) {
    const [opponent, me] = <[Code, Code]>line.split(' ')
    score += evaluate(map[opponent], map[me])
}

console.log(score)
