// let filepath = "../../inputs/D1/small_input2.txt"
let filepath = "../../inputs/D2/main_input.txt"

let verbose = true

let inspect =
    (fun any ->
        if verbose then
            any |> printfn "%A"

        any)

let lines = System.IO.File.ReadAllLines filepath
// lines |> Array.iter(fun line -> printfn "%s" line)

type Show(r: int, g: int, b: int) =
    member val r = r with get, set
    member val g = g with get, set
    member val b = b with get, set

    override this.ToString() = sprintf "(r: %d, g: %d, b: %d)" r g b

    member this.lt(other: Show) =
        (r < other.r && g < other.g && b < other.b)

    new() = Show(0, 0, 0)

let part1 =
    fun (lines: string[]) ->
        lines
        |> Array.mapi (fun (i: int) (line: string) ->
            let cubes = line[(line.IndexOf(':') + 2) ..]
            let mutable show = Show()

            cubes.Split(';')
            |> Array.map (fun subArr -> subArr.Trim().Split(',') |> Array.map (fun s -> s.Trim()))
            |> Seq.concat
            |> Seq.toArray
            // |> inspect
            |> fun arr -> (show, arr)
            ||> Array.fold (fun acc single ->
                let l = single.Split(" ")

                (match l[1] with
                 | "red" ->
                     l[0]
                     |> System.Int32.Parse
                     |> (fun n -> if n > acc.r then Show(n, acc.g, acc.b) else (acc))
                 | "green" ->
                     l[0]
                     |> System.Int32.Parse
                     |> (fun n -> if n > acc.g then Show(acc.r, n, acc.b) else (acc))
                 | "blue" ->
                     l[0]
                     |> System.Int32.Parse
                     |> (fun n -> if n > acc.b then Show(acc.r, acc.g, n) else (acc))
                 | _ -> (acc)

                )
            // |> ignore
            )
            // |> inspect
            // |> ignore

            // show

        )
// |> inspect

// inspect line
let max = Show(12, 13, 14)

lines 
|> part1 
|> inspect 
|> Array.mapi(fun i show -> 
    if show.lt(max) then i + 1 else 0
)
|> inspect 
// |> Array.length
|> Array.sum
|> inspect
|> ignore

// lines
// |> Array.mapi (fun i line ->
//     part1 line i
// )
// |> inspect
// |> ignore
