module Spiral
  ( spiral
  ) where

import Prelude
import Data.List (
    (:)
    , drop
    , fromFoldable
    , List(Nil)
    , range
    , reverse
    , take
    , transpose
    )
import Data.Tuple (Tuple(Tuple))

spiral :: Int -> List (List Int)
spiral n = rectangle (range 1 (n * n)) n n

rectangle :: forall a. List a -> Int -> Int -> List (List a)
rectangle _ _ 0 = Nil
rectangle x j k = y : rotate (rectangle z k $ j - 1) where Tuple y z = split k x

rotate :: forall a. List (List a) -> List (List a)
rotate = transpose <<< reverse

split :: forall a. Int -> List a -> Tuple (List a) (List a)
split n x = Tuple (take n x) (drop n x)
