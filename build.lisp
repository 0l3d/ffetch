#!/usr/bin/env -S sbcl --script

(require :sb-posix)

(defun script-directory ()
  (namestring
   (make-pathname :name nil
                  :type nil
                  :defaults (or *load-truename* *load-pathname* #P"./build.lisp"))))

(let* ((project-root (script-directory))
       (target-dir (namestring (merge-pathnames #P"target/" project-root))))
  (sb-ext:setenv "CARGO_TARGET_DIR" target-dir)
  (sb-posix:chdir project-root)
  (let* ((process
           (sb-ext:run-program
            "cargo"
            '("build" "--locked")
            :search t
            :input *standard-input*
            :output *standard-output*
            :error *error-output*))
         (status (sb-ext:process-exit-code process)))
    (sb-ext:exit :code status)))
