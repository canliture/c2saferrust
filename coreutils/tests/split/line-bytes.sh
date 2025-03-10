#!/bin/sh
# test -C, --lines-bytes

# Copyright (C) 2013-2024 Free Software Foundation, Inc.

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
. "$SCRIPTPATH/../../tests/init.sh"; path_prepend_ $1

# Ensure memory is not allocated up front

vm=$(get_min_ulimit_v_ split -C 'K' /dev/null) && {
  (ulimit -v $vm && split -C 'G' /dev/null) || fail=1
}


# Ensure correct operation with various split and buffer size combinations

lines=\
1~2222~3~4

printf '%s' "$lines" | tr '~' '\n' > in || framework_failure_

cat <<\EOF > splits_exp
1 1 1 1 1 1 1 1 1 1
2 2 2 1 2 1
2 3 2 2 1
2 4 3 1
2 5 3
2 5 3
7 3
7 3
9 1
9 1
10
EOF

seq 0 9 | tr -d '\n' > no_eol_in

cat <<\EOF > no_eol_splits_exp
1 1 1 1 1 1 1 1 1 1
2 2 2 2 2
3 3 3 1
4 4 2
5 5
6 4
7 3
8 2
9 1
10
10
EOF

for b in $(seq 10); do
  > splits
  > no_eol_splits
  for s in $(seq 11); do
    rm x??
    split ---io=$b -C$s in || fail=1
    cat x* > out || framework_failure_
    compare in out || fail=1
    stat -c %s x* | paste -s -d ' ' >> splits

    rm x??
    split ---io=$b -C$s no_eol_in || fail=1
    cat x* > out || framework_failure_
    cat xaa
    compare no_eol_in out || fail=1
    stat -c %s x* | paste -s -d ' ' >> no_eol_splits
  done
  compare splits_exp splits || fail=1
  compare no_eol_splits_exp no_eol_splits || fail=1
done

# Test hold buffer management with --lines-bytes.
# The following triggers (with ASAN) a heap overflow issue
# between coreutils 9.2 and 9.4 inclusive.
printf '%131070s\n' '' >expaa || framework_failure_
printf 'x\n' >expab || framework_failure_
printf '%131071s\n' '' >expac || framework_failure_
cat expaa expab expac >bigin || framework_failure_
split -C 131072 ---io=131072 bigin || fail=1
compare expaa xaa || fail=1
compare expab xab || fail=1
compare expac xac || fail=1

Exit $fail
