#!/usr/bin/env bash
set -euo pipefail

# Run this inside the provided filler container where maps and robots exist.
# Supports both naming variants:
# - game_engine + robots
# - linux_game_engine + linux_robots
# Example:
#   ./scripts/audit_runner.sh /filler/solution/bin/filler

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <path-to-student-player-binary>"
  exit 1
fi

PLAYER_PATH="$1"
ENGINE=""
ROBOT_DIR=""

if [[ -x "./game_engine" ]]; then
  ENGINE="./game_engine"
  ROBOT_DIR="robots"
elif [[ -x "./linux_game_engine" ]]; then
  ENGINE="./linux_game_engine"
  ROBOT_DIR="linux_robots"
else
  echo "[error] No supported engine found. Expected ./game_engine or ./linux_game_engine"
  exit 1
fi

if [[ ! -x "${PLAYER_PATH}" ]]; then
  echo "[error] Player binary not executable: ${PLAYER_PATH}"
  exit 1
fi

if [[ ! -d "maps" ]] || [[ ! -d "${ROBOT_DIR}" ]]; then
  echo "[error] maps/ or ${ROBOT_DIR}/ not found in current directory."
  exit 1
fi

mkdir -p audit_logs

winner_of_log() {
  local log_file="$1"

  # Common winner strings.
  if grep -Eiq "player[[:space:]]*1[[:space:]]*(wins|won)|winner[^0-9]*1|p1[[:space:]]*(wins|won)" "$log_file"; then
    echo "p1"
    return 0
  fi
  if grep -Eiq "player[[:space:]]*2[[:space:]]*(wins|won)|winner[^0-9]*2|p2[[:space:]]*(wins|won)" "$log_file"; then
    echo "p2"
    return 0
  fi

  # Try to parse score line formats with two final integers.
  local score_line
  score_line="$(grep -Ei "(score|final|fin|result)" "$log_file" | tail -n 1 || true)"
  if [[ -n "$score_line" ]]; then
    local nums
    nums=( $(echo "$score_line" | grep -Eo '[0-9]+' || true) )
    if [[ ${#nums[@]} -ge 2 ]]; then
      if (( nums[0] > nums[1] )); then
        echo "p1"
        return 0
      elif (( nums[1] > nums[0] )); then
        echo "p2"
        return 0
      fi
    fi
  fi

  echo "unknown"
}

run_series() {
  local map="$1"
  local enemy="$2"
  local label="$3"

  local wins=0
  local total=0

  echo ""
  echo "=== ${label} (${map} vs ${enemy}) ==="

  # 5 runs, swapping sides each run to cover p1/p2 cases.
  for i in 1 2 3 4 5; do
    local p1
    local p2
    local student_side

    if (( i % 2 == 1 )); then
      p1="${PLAYER_PATH}"
      p2="${ROBOT_DIR}/${enemy}"
      student_side="p1"
    else
      p1="${ROBOT_DIR}/${enemy}"
      p2="${PLAYER_PATH}"
      student_side="p2"
    fi

    local log_file="audit_logs/${label}_${i}.log"
    echo "[run ${i}] map=${map} student=${student_side}"

    set +e
    "${ENGINE}" -q -f "maps/${map}" -p1 "$p1" -p2 "$p2" > "$log_file" 2>&1
    local rc=$?
    set -e

    if [[ $rc -ne 0 ]]; then
      echo "  -> engine exited with code ${rc} (counted as loss)."
      total=$((total + 1))
      continue
    fi

    local winner
    winner="$(winner_of_log "$log_file")"

    local student_won=0
    if [[ "$winner" == "p1" && "$student_side" == "p1" ]]; then
      student_won=1
    elif [[ "$winner" == "p2" && "$student_side" == "p2" ]]; then
      student_won=1
    fi

    if (( student_won == 1 )); then
      wins=$((wins + 1))
      echo "  -> student WIN"
    else
      echo "  -> student LOSS (parsed winner: ${winner})"
    fi

    total=$((total + 1))
  done

  echo "[summary] ${label}: ${wins}/${total} wins"
  if (( wins >= 4 )); then
    echo "[audit] PASS (>= 4/5)"
  else
    echo "[audit] FAIL (< 4/5)"
  fi
}

run_series "map00" "wall_e" "map00_vs_wall_e"
run_series "map01" "h2_d2" "map01_vs_h2_d2"
run_series "map02" "bender" "map02_vs_bender"

echo ""
echo "Logs saved in ./audit_logs"
