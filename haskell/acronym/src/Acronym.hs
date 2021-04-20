module Acronym (abbreviate) where

import qualified Data.Text as T
import Data.Text (Text)

abbreviate :: Text -> Text
abbreviate xs =  T.split (\c -> c == ' ' || c == '-' || c== '_') xs
