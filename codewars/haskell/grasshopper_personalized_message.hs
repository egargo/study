-- https://www.codewars.com/kata/5772da22b89313a4d50012f7/train/haskell


module PersonalisedGreeting (greet) where

greet :: String -> String ->String
greet name owner =
    if name == owner
        then "Hello boss"
    else "Hello guest"