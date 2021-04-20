module LeapYear (isLeapYear) where

isDiv :: Integer -> Integer -> Bool
isDiv num den = mod num den == 0

isLeapYear :: Integer -> Bool
isLeapYear year = leapMatch (isDiv year 4, isDiv year 100, isDiv year 400)

leapMatch :: (Bool, Bool, Bool) -> Bool
leapMatch (False, _, _) = False
leapMatch (True, True, False) = False
leapMatch (True, True, True) = True
leapMatch (True, False, _) = True
