infile = "input.txt"

part1 xs = 
    head $ [l * r | l <- xs, r <- xs, l + r == 2020]

part2 xs = head $ [l * r * t | l <- xs, r <- xs, t <-xs, l + r + t == 2020]
main = do
    content <- readFile infile
    let xs = map (\x -> read x :: Integer) . lines $ content
    mapM_ print $ map ($ xs) [part1, part2]
    