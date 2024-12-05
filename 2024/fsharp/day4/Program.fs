open System
let filepath = "../../inputs/D4/small.txt"
// let filepath = "../../inputs/D4/main.txt"

let verbose = true

let inspect =
    (fun any ->
        if verbose then
            any |> printfn "%A"

        any)

let lines = System.IO.File.ReadAllLines filepath

let part1 =
    fun (lines: string array) ->
        let mat  = lines |> Array.map (fun line -> line.Split "")
        let width = mat[0].Length
        let height = lines.Length
        let verts = [0..height - 4] |> Array.filter (fun j -> 
            [0..width] |> Array.filter (fun i -> 
                let l = [0..3] |> Array.map(fun a -> mat[j + a][i])
                printfn "(%A, %A) : %A" i j l
                l = [|"X", "M", "A", "S"|] || l = [|"S", "A", "M", "X"|]
            )
            true
        )
        verts


part1 lines
|> inspect
|> ignore