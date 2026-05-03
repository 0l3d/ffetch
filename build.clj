#!/usr/bin/env clojure

(ns build
  (:require [clojure.java.io :as io])
  (:import [java.lang ProcessBuilder]))

(def project-root
  (.getCanonicalFile (io/file ".")))

(def target-dir
  (.getAbsolutePath (io/file project-root "target")))

(def builder
  (doto (ProcessBuilder. ["cargo" "build" "--locked"])
    (.directory project-root)
    (.inheritIO)))

(.put (.environment builder) "CARGO_TARGET_DIR" target-dir)

(let [process (.start builder)]
  (System/exit (.waitFor process)))
