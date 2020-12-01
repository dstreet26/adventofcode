module Main where

import Data.Set (Set)
import qualified Data.Set as Set
import Data.List.Split
import Math.Combinatorics.Digraph

main :: IO ()
main = do
  contents <- readFile "input.txt"
  let array = words $ contents
  let bodies = getBodies array
  print $ Set.size bodies
  -- print . map readInt . words $ contents

          -- print . sum . map process2 . map readInt $ ["100756"]
            -- print . sum . map process2 . map readInt . words $ contents

readInt :: String -> Int
readInt = read

-- readPair :: String -> (String, String)
-- readPair = read

getBodies :: [String] -> Set String
getBodies = Set.fromList . concatMap (\x -> splitOn ")" x)

-- DG [v] [(v, v)]
getDigraph :: [String] -> [(String, String)] -> DG

process :: Int -> Int
process x = (floor ((fromIntegral x) / (3))) - 2

process2 :: Int -> Int
process2 x =
  let processed = process x
    in if processed <= 0
        then 0
        else processed + (process2 processed)
