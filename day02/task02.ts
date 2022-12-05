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
    A: Sign.Rock,
    B: Sign.Paper,
    C: Sign.Scissors,
}

const sign_score = {
    [Sign.Rock]: 1,
    [Sign.Paper]: 2,
    [Sign.Scissors]: 3,
}

type Strategy = 'X' | 'Y' | 'Z' // Lose | Draw | Win

const strategy_map: Record<Strategy, Record<Sign, Sign>> = {
    X: {
        [Sign.Rock]: Sign.Scissors,
        [Sign.Paper]: Sign.Rock,
        [Sign.Scissors]: Sign.Paper,
    },
    Y: {
        [Sign.Rock]: Sign.Rock,
        [Sign.Paper]: Sign.Paper,
        [Sign.Scissors]: Sign.Scissors,
    },
    Z: {
        [Sign.Rock]: Sign.Paper,
        [Sign.Paper]: Sign.Scissors,
        [Sign.Scissors]: Sign.Rock,
    },
}

function get_answer_sign(opponent: Sign, strategy: Strategy): Sign {
    return strategy_map[strategy][opponent]
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
    const [opponent, me] = <[Code, Strategy]>line.split(' ')
    const should_sign = get_answer_sign(map[opponent], me)
    score += evaluate(map[opponent], should_sign)
}

console.log(score)
