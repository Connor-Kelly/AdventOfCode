open System.Text.RegularExpressions
open System
// let filepath = "../../inputs/D3/small.txt"
// let filepath = "../../inputs/D3/small2.txt"
let filepath = "../../inputs/D3/main.txt"

let verbose = true

let inspect =
    (fun any ->
        if verbose then
            any |> printfn "%A"

        any)

let lines = System.IO.File.ReadAllLines filepath

let part1 =
    fun (lines: string array) ->
        lines
        |> Array.map (fun line ->
            let reg = Regex("mul\([0-9]+,[0-9]+\)")
            // let reg = Regex("mul")
            line
            |> reg.Matches
            |> (fun m ->
                m.Count |> ignore
                m)
            |> Seq.map (fun m ->
                let str = m.Value[4 .. m.Value.Length - 2]
                let split = str.Split(",")
                Int32.Parse(split[0]) * Int32.Parse(split[1]))
            |> Seq.sum)
        |> Array.sum

// part1 lines
// |> inspect
// |> ignore

let part2 =
    fun (lines: string array) ->
        let mutable enabled = true
        lines
        |> Array.map (fun line ->
            let reg = Regex("do(n't)?\(\)|mul\([0-9]+,[0-9]+\)")
            // let reg = Regex("mul")

            line
            |> reg.Matches
            |> (fun m ->
                m.Count |> ignore
                m)
            |> Seq.map (fun m ->
                m.Value |> inspect

                match m.Value with
                | "don't()" ->
                    (enabled <- false
                     0)
                | "do()" ->
                    (enabled <- true
                     0)
                | mul ->
                    (let str = mul[4 .. mul.Length - 2]
                     let split = str.Split(",")
                     printfn "enabled %A | mul: %A" enabled str

                     if (enabled) then
                         Int32.Parse(split[0]) * Int32.Parse(split[1])
                     else
                         0))
            |> Seq.sum)

        |> Array.sum

part2 lines |> inspect |> ignore
