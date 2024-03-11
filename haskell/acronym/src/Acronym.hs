module Acronym (abbreviate) where

import Data.Char (isAlpha, isLower, isUpper, toUpper)

m :: (Char, Char) -> String
m (x, y) | not (isAlpha x || x == '\'') && isAlpha y = [toUpper y]
m (x, y) | isLower x && isUpper y = [toUpper y]
m _ = []

abbreviate :: String -> String
abbreviate s = concatMap m $ zip (" " <> s) s
