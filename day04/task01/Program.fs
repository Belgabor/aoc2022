// For more information see https://aka.ms/fsharp-console-apps
let splitAssignment (assignment: string) = 
    assignment.Split [|'-'|] |> Array.toList |> List.map int

let isContained (a: int list, b: int list) =
    (a[0] >= b[0]) && (a[1] <= b[1])

let processLine (line: string) = 
    let assignments = line.Split [|','|] |> Array.toList
    let regions = List.map (fun assignment -> splitAssignment assignment) assignments
    let contained = isContained (regions[0], regions[1]) || isContained (regions[1], regions[0])
    if contained then 1 else 0



[<EntryPoint>]
let main _ =
    let lines = System.IO.File.ReadAllLines("../data.txt") |> Array.toList
    let score = List.fold(fun acc elem -> acc + processLine elem) 0 lines
    printfn "%i" score
    0
