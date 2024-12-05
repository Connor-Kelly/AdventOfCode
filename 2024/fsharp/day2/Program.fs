open System
// let filepath = "../../inputs/D2/small_input.txt"
let filepath = "../../inputs/D2/main_input.txt"

let verbose = true

let inspect =
    (fun any ->
        if verbose then
            any |> printfn "%A"

        any)

let lines = System.IO.File.ReadAllLines filepath

let is_safe =
    fun (line: int array) ->
        let diff = (Array.zip line[0 .. line.Length - 2] (line[1..])
             |> Array.map (fun (a, b) -> a - b))

        let within_bounds =
            diff
            |> Array.forall (fun (elem: int) ->
                let e = Math.Abs elem
                e >= 1 && e <= 3)

        // printfn "within: "
        // within_bounds |> inspect |> ignore

        let is_monotonic =
            diff |> Array.forall (fun elem -> elem > 0)
            // |> inspect
            || diff |> Array.forall (fun elem -> elem < 0)
        // |> inspect

        is_monotonic && within_bounds

let part1 =
    fun (lines: string array) ->
        lines
        |> Array.map (fun line -> line.Split " " |> Array.map (fun str -> System.Int32.Parse str))
        |> Array.map (fun line ->
            // let diff =
            //     (Array.zip line[0 .. line.Length - 2] (line[1..])
            //      |> Array.map (fun (a, b) -> a - b))

            // inspect diff |> ignore
            is_safe line)
        |> inspect
        |> Array.filter (fun b -> b)
        |> (fun a -> a.Length)
        |> inspect

part1 lines


let part2 =
    fun (lines: string array) ->
        lines
        |> Array.map (fun line -> line.Split " " |> Array.map (fun str -> System.Int32.Parse str))
        |> Array.map (fun line ->

            if is_safe line then
                true
            else
                [ 0 .. line.Length - 1 ]
                |> List.exists (fun i -> (i, line) ||> Array.removeAt |> is_safe)

        // is_safe diff

        // is_monotonic && wikthin_bounds
        )
        |> inspect
        |> Array.filter (fun b -> b)
        |> (fun a -> a.Length)
        |> inspect

part2 lines
