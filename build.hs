#!/usr/bin/env runghc
{-# LANGUAGE CPP #-}

import System.Directory (setCurrentDirectory)
import System.Environment (getEnvironment)
import System.Exit (exitWith)
import System.FilePath ((</>), takeDirectory)
import System.Process (CreateProcess(..), StdStream(..), createProcess, proc, waitForProcess)

main :: IO ()
main = do
  let projectRoot = takeDirectory __FILE__
      targetDir = projectRoot </> "target"

  environment <- getEnvironment
  setCurrentDirectory projectRoot

  (_, _, _, processHandle) <-
    createProcess
      (proc "cargo" ["build", "--locked"])
        { env = Just (("CARGO_TARGET_DIR", targetDir) : environment),
          std_in = Inherit,
          std_out = Inherit,
          std_err = Inherit
        }

  waitForProcess processHandle >>= exitWith
