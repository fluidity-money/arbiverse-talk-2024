#!/bin/sh

case "$1" in
	part-1) cargo test -- test_is_pet;;
	*) exit 1 ;;
esac

