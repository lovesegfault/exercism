module Acronym (abbreviate) where

import Data.Char (isSpace, isUpper)
import Data.Text (Text)
import qualified Data.Text as T

abbreviate :: Text -> Text
abbreviate =
  T.toUpper
    . T.concat
    . map (\w -> T.take 1 w <> (T.filter isUpper . T.dropWhile isUpper) w)
    . T.split (\c -> isSpace c || c `elem` ['-', '_'])
