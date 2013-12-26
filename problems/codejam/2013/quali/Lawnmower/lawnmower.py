import sys, time

def get_data():
    numcases = int(sys.stdin.readline())
    for case in range(1,numcases+1):
        N, M = [int(v) for v in sys.stdin.readline().strip().split()]
        data = []
        for line_number in range(N):
            line = sys.stdin.readline().strip().split()
            data.append(line)
        yield (case, data)

def below_max_height(lawn, max_height = 100):
  for line in lawn:
    if any(1 <= field <= max_height for field in line):
      return "NO"
  return "YES"

def solve(lawn):
  # Take highest setting as pattern
  heights = [max(line) for line in lawn]
  max_line = heights.index(max(heights))
  pattern = lawn[max_line]

  # Check all other lines against this pattern
  other_lines = lawn[:max_line] + lawn[max_line+1:]
  for line in other_lines:
    if line != pattern:
      setting = max(line)
      trimmed = [min(setting, f) for f in line]
      # All fields which are lower than the setting
      # must have the same height as the pattern
      if any(t < setting and t != p for t in trimmed for p in pattern):
          return "NO"
  return below_max_height(lawn)

def main():
    for case, data in get_data():
      print "Case #" + repr(case) + ":", solve(data)
      for line in data:
        print "".join(line)
      print

if __name__ == "__main__":
  main()
