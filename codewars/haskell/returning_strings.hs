-- https://www.codewars.com/kata/55a70521798b14d4750000a4/train/haskell

module Greeting where

greeting :: String -> String
greeting name =
  "Hello, " ++ name ++ " how are you doing today?"

main = do
  print (greeting "bee")
