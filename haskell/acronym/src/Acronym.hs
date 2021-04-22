module Acronym (abbreviate) where

import Data.Char (isSpace, isUpper)
import Data.Text (Text)
import qualified Data.Text as T

abbreviate :: Text -> Text
abbreviate =
  T.toUpper
    . T.concat
    . map (T.take 1 <> T.filter isUpper . T.dropWhile isUpper)
    . T.split (\c -> isSpace c || c `elem` ['-', '_'])
