module PerfectNumbers (classify, Classification (..)) where

data Classification = Deficient | Perfect | Abundant deriving (Eq, Show)

aliquotSum :: Int -> Int
aliquotSum n =
  let limit = (n `div` 2)
   in sum . filter (\i -> n `mod` i == 0) $ [1 .. limit]

classify :: Int -> Maybe Classification
classify n =
  if n <= 0
    then Nothing
    else case compare n (aliquotSum n) of
      EQ -> Just Perfect
      GT -> Just Deficient
      LT -> Just Abundant
