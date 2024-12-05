// let filepath = "../../inputs/D1/small_input2.txt"
let filepath = "../../inputs/D1/main_input.txt"

let verbose = true

let inspect =
    (fun any ->
        if verbose then
            any |> printfn "%A"

        any)

let lines = System.IO.File.ReadAllLines filepath


let part1 = fun (lines : string array) -> 
    (([], []), lines)
    ||> Array.fold (fun (a, b) line -> 
            let nums = line.Split("   ");
            (
                List.append a  [nums[0] |> System.Int32.Parse], 
                List.append b [nums[1] |> System.Int32.Parse])
        )
    
        |> inspect 
        |> (fun (a, b) -> (List.sort a, List.sort b))
        |> inspect 
        |> (fun (a, b) -> List.zip a b)
        |> inspect 
        |> List.map(fun (a, b) -> System.Math.Abs(a - b))
        |> inspect
        |> List.sum
        |> inspect
        |> ignore

// part1 lines

let part2 = fun (lines: string array) ->
    (([], []), lines)
        ||> Array.fold (fun (a, b) line ->
        let nums = line.Split("   ")
        (List.append a [ nums[0] |> System.Int32.Parse ], List.append b [ nums[1] |> System.Int32.Parse ]))
    |> fun (a, b ) -> 
        a |> List.map (fun i -> 
            (b |> List.filter (fun j -> i = j)).Length * i
        )
    |> inspect 
    |> List.sum
    |> inspect 
    |> ignore

part2 lines
// |> Array.map (fun line ->
//     let digits =
//         [| 0 .. line.Length - 1 |]
//         |> Array.map (fun i -> line[i..])
//         |> inspect
//         |> Array.map (fun s ->
//             match s with
//             | _ when s.Length > 0 && System.Char.IsDigit s[0] -> Some(System.Int32.Parse(s[0].ToString()))
//             | s when s.Length >= "one".Length && s.StartsWith("one") -> Some 1
//             | s when s.Length >= "two".Length && s.StartsWith("two") -> Some 2
//             | s when s.Length >= "three".Length && s.StartsWith("three") -> Some 3
//             | s when s.Length >= "four".Length && s.StartsWith("four") -> Some 4
//             | s when s.Length >= "five".Length && s.StartsWith("five") -> Some 5
//             | s when s.Length >= "six".Length && s.StartsWith("six") -> Some 6
//             | s when s.Length >= "seven".Length && s.StartsWith("seven") -> Some 7
//             | s when s.Length >= "eight".Length && s.StartsWith("eight") -> Some 8
//             | s when s.Length >= "nine".Length && s.StartsWith("nine") -> Some 9
//             | s when s.Length >= "zero".Length && s.StartsWith("zero") -> Some 0
//             | _ -> None)
//         |> Array.filter (fun o -> o.IsSome)
//         |> Array.map (fun some -> some.Value)
//         |> inspect

//     let combo_string = sprintf "%i%i" digits[0] digits[digits.Length - 1]
//     inspect combo_string |> ignore

//     System.Int32.Parse(combo_string))
// |> inspect
// |> Array.sum
// // |> inspect
// |> printfn "Sum: %d"
// |> ignore
