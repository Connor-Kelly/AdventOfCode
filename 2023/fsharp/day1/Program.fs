// let filepath = "../../inputs/D1/small_input2.txt"
let filepath = "../../inputs/D1/main_input.txt"

let verbose = true

let inspect =
    (fun any ->
        if verbose then
            any |> printfn "%A"

        any)

let lines = System.IO.File.ReadAllLines filepath

lines
|> Array.map (fun line ->
    let digits =
        [| 0 .. line.Length - 1 |]
        |> Array.map (fun i -> line[i..])
        |> inspect
        |> Array.map (fun s ->
            match s with
            | _ when s.Length > 0 && System.Char.IsDigit s[0] -> Some(System.Int32.Parse(s[0].ToString()))
            | s when s.Length >= "one".Length && s.StartsWith("one") -> Some 1
            | s when s.Length >= "two".Length && s.StartsWith("two") -> Some 2
            | s when s.Length >= "three".Length && s.StartsWith("three") -> Some 3
            | s when s.Length >= "four".Length && s.StartsWith("four") -> Some 4
            | s when s.Length >= "five".Length && s.StartsWith("five") -> Some 5
            | s when s.Length >= "six".Length && s.StartsWith("six") -> Some 6
            | s when s.Length >= "seven".Length && s.StartsWith("seven") -> Some 7
            | s when s.Length >= "eight".Length && s.StartsWith("eight") -> Some 8
            | s when s.Length >= "nine".Length && s.StartsWith("nine") -> Some 9
            | s when s.Length >= "zero".Length && s.StartsWith("zero") -> Some 0
            | _ -> None)
        |> Array.filter (fun o -> o.IsSome)
        |> Array.map (fun some -> some.Value)
        |> inspect

    let combo_string = sprintf "%i%i" digits[0] digits[digits.Length - 1]
    inspect combo_string |> ignore

    System.Int32.Parse(combo_string))
|> inspect
|> Array.sum
// |> inspect
|> printfn "Sum: %d"
|> ignore
