# Shared helpers for the gate scripts in scripts/. Read with `. scripts/lib.sh`, never run directly.
# Design: docs/specs/_root/0008-build-dan-penerimaan-per-target/index.md (Kerangka lib.sh, Bentuk keluaran).
# Written for bash 3.2 (the bash that ships with macOS) up to 5.x: no arrays, no mapfile.

STEPS=0
FAILED=0
FAILED_STEPS=""
NAME=$(basename "$0")

# no_args <argument count>: any argument prints the usage line to stderr and exits with code 2.
no_args() {
  if [ "$1" -ne 0 ]; then
    echo "usage: bash scripts/$NAME" >&2
    exit 2
  fi
}

# needs_darwin_arm64: scripts for the macOS app stop with code 2 on any other host, before the header.
needs_darwin_arm64() {
  host_os=$(uname -s)
  host_arch=$(uname -m)
  if [ "$host_os" != "Darwin" ] || [ "$host_arch" != "arm64" ]; then
    echo "$NAME: needs macOS on arm64, this host is $host_os $host_arch" >&2
    exit 2
  fi
}

# must_print_nothing <command> [arguments...]: run the command, pass its output on, and fail if it printed anything.
must_print_nothing() {
  quiet_out=$("$@" 2>&1)
  quiet_code=$?
  [ -n "$quiet_out" ] && printf '%s\n' "$quiet_out"
  [ "$quiet_code" -eq 0 ] || return "$quiet_code"
  [ -z "$quiet_out" ]
}

# summary_line <text>: append one line to the GitHub job summary when it is set.
summary_line() {
  if [ -n "${GITHUB_STEP_SUMMARY:-}" ]; then
    printf '%s\n' "$1" >> "$GITHUB_STEP_SUMMARY"
  fi
}

# summary_row <n> <label> <result>: one table row in the job summary, with every | in the label written \|.
summary_row() {
  if [ -n "${GITHUB_STEP_SUMMARY:-}" ]; then
    row_label=$(printf '%s' "$2" | sed 's/|/\\|/g')
    printf '| %s | `%s` | %s |\n' "$1" "$row_label" "$3" >> "$GITHUB_STEP_SUMMARY"
  fi
}

# header: print the header lines, and write the job summary title, header, and table head.
header() {
  if [ -z "$(git status --porcelain 2>/dev/null)" ]; then tree=clean; else tree=dirty; fi
  head_lines="== $NAME
host: $(uname -s) $(uname -m)
commit: $(git rev-parse HEAD 2>&1) ($tree)
rustc: $(rustc --version 2>&1)
cargo: $(cargo --version 2>&1)"
  if [ "$(uname -s)" = "Darwin" ]; then
    head_lines="$head_lines
macos: $(sw_vers -productVersion 2>&1)
xcode: $(xcodebuild -version 2>&1 | head -n 2 | tr '\n' ' ' | sed 's/ $//')
swift: $(swift --version 2>&1 | head -n 1)"
  fi
  printf '%s\n' "$head_lines"
  summary_line "### $NAME"
  summary_line ""
  summary_line '```text'
  summary_line "$head_lines"
  summary_line '```'
  summary_line ""
  summary_line "| Step | Command | Result |"
  summary_line "| --- | --- | --- |"
}

# step <label> <command> [arguments...]: run the command, merge stderr into stdout, record the result.
step() {
  label=$1; shift
  STEPS=$((STEPS + 1))
  echo "-- step $STEPS: $label"
  start=$SECONDS
  "$@" 2>&1
  code=$?
  if [ "$code" -eq 0 ]; then result=PASS; else result=FAIL; FAILED=$((FAILED + 1)); FAILED_STEPS="$FAILED_STEPS $STEPS"; fi
  echo "-- step $STEPS: $result (exit $code, $((SECONDS - start))s)"
  summary_row "$STEPS" "$label" "$result"
  return "$code"
}

# logged <file> <command> [arguments...]: run the command while keeping a copy in target/scripts/<file>.
logged() {
  mkdir -p target/scripts
  log_file=target/scripts/$1; shift
  "$@" 2>&1 | tee "$log_file"
}

# toolchain_channel: the channel pinned in rust-toolchain.toml.
toolchain_channel() {
  sed -n 's/^channel *= *"\([^"]*\)".*/\1/p' rust-toolchain.toml | tr -d '\r'
}

# rustc_matches_channel <channel>: print rustc --version and pass only if it starts with "rustc <channel> ".
rustc_matches_channel() {
  rustc_line=$(rustc --version 2>&1)
  rustc_code=$?
  printf '%s\n' "$rustc_line"
  [ "$rustc_code" -eq 0 ] || return "$rustc_code"
  case "$rustc_line" in
    "rustc $1 "*) return 0 ;;
    *) return 1 ;;
  esac
}

# toolchain_step: step 1 of every script.
toolchain_step() {
  channel=$(toolchain_channel)
  step "toolchain matches rust-toolchain.toml ($channel)" rustc_matches_channel "$channel"
}

# rust_tests_line: sum of the passed, failed, and ignored counts in target/scripts/cargo-test.log.
rust_tests_line() {
  if grep -q 'test result:' target/scripts/cargo-test.log 2>/dev/null; then
    grep 'test result:' target/scripts/cargo-test.log | awk '
      { for (i = 2; i <= NF; i++) {
          if ($i ~ /^passed/) p += $(i - 1)
          if ($i ~ /^failed/) f += $(i - 1)
          if ($i ~ /^ignored/) g += $(i - 1)
        } }
      END { printf "rust tests: %d passed, %d failed, %d ignored\n", p, f, g }'
  else
    echo "rust tests: no result lines"
  fi
}

# finish: print the extra lines of the script (function extra_lines, if defined), the closing line, and exit 0 or 1.
finish() {
  if type extra_lines >/dev/null 2>&1; then
    extra_lines
  fi
  if [ "$FAILED" -eq 0 ]; then
    closing="== $NAME: PASS, $STEPS steps, 0 failed"
    exit_code=0
  else
    closing="== $NAME: FAIL, $STEPS steps, $FAILED failed (steps$FAILED_STEPS)"
    exit_code=1
  fi
  summary_line ""
  summary_line "$closing"
  echo "$closing"
  exit "$exit_code"
}
