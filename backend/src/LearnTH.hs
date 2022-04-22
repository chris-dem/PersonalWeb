{-# LANGUAGE TemplateHaskell #-}

module LearnTH where

import Control.Monad
import Language.Haskell.TH


curryN :: Int -> Q Exp
curryN n = do
    f  <- newName "f"
    xs <- replicateM n (newName "x")
    let args = map VarP (f:xs)
    let ntup = TupE (map (pure . VarE) xs)
    return $ LamE args (AppE (VarE f) ntup)