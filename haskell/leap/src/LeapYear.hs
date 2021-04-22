module LeapYear (isLeapYear) where

isDiv :: Integer -> Integer -> Bool
isDiv num den = mod num den == 0

isLeapYear :: Integer -> Bool
isLeapYear year = case (year `isDiv` 4, year `isDiv` 100, year `isDiv` 400) of
  (False, _, _) -> False
  (True, True, False) -> False
  (True, True, True) -> True
  (True, False, _) -> True
