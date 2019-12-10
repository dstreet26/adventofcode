module Main where

main :: IO ()
main = do
  contents <- readFile "input.txt"
  print . sum . map process2 . map readInt . words $ contents

readInt :: String -> Int
readInt = read

process :: Int -> Int
process x = (floor ((fromIntegral x) / (3))) - 2

process2 :: Int -> Int
process2 x =
  let processed = process x
   in if processed <= 0
        then 0
        else processed + (process2 processed)
