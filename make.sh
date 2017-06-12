#!/bin/sh -e

MODULES="      \
  anagram      \
  bob          \
  etl          \
  hello-world  \
  word-count   \
"

TASK=${1:-"all"}

task_check() {
  echo "nothing to check, yet!"
}

task_test() {
  task_check

  cargo test
}

task_all() {
  task_test
}

task_clean() {
  cargo clean
}

for MODULE in $MODULES
do
  cd $MODULE
  case $TASK in
    check)
      task_check
      ;;
    test)
      task_test
      ;;
    all)
      task_all
      ;;
    clean)
      task_clean
      ;;
  esac
  cd ../
done
