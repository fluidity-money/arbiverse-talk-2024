#!/bin/sh

case "$1" in
	example) cargo test -- test_contract_fizzbuzzer ;;
	*) exit 1 ;;
esac
