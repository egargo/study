-- https://www.codewars.com/kata/5ab6538b379d20ad880000ab/train/haskell


module AreaPerimeter where

areaOrPerimeter :: Double -> Double -> Double
areaOrPerimeter l w =
    if l == w
        then l * w
    else (2 * l) + (2 * w)