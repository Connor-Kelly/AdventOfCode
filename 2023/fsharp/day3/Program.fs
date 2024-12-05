// let filepath = "../../inputs/D1/small_input2.txt"
let filepath = "../../inputs/D3/small_input.txt"

let verbose = true

let inspect =
    (fun any ->
        if verbose then
            any |> printfn "%A"

        any)

let lines = System.IO.File.ReadAllLines filepath
// lines |> Array.iter(fun line -> printfn "%s" line)

let part1 =
    fun (lines: string[]) ->
        lines |> Array.map (fun line -> 
            let mutable start = 0
            line |> inspect |> ignore
            line.ToCharArray () |> Array.iteri (fun i char ->
                match (char) with
                | '.' -> (line[start..i-1] |> inspect |> ignore ;
                    start <- i + 1)
                | _ -> ()
            )

            line
        )


lines 
|> part1 
|> inspect 
|> ignore

