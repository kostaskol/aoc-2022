def main
  filename =
    if ARGV.any? { |a| a == '-t' }
      'input/test/day10.in'
    else
      'input/day10.in'
    end
  lines = File.read(filename)

  if ARGV.any? { |a| a == '-e' }
    solve2(lines)
  else
    solve1(lines)
  end
end

def solve2(lines)
  cycle = 0
  regx = 1
  lit = []

  in_sync = lambda { |c, x| (((c - 1) % 40) - x).abs <= 1 }

  lines.split("\n").each do |line|
    cycle += 1
    lit << cycle - 1 if in_sync.call(cycle, regx)

    cmd, val = line.split(' ')

    next if cmd == 'noop'

    cycle += 1
    lit << cycle - 1 if in_sync.call(cycle, regx)
    regx += val.to_i
  end

  puts lit.join(',')
  s = Array.new(6) { |_| Array.new(40) { |_| '.' } }
  lit.each do |l|
    row = l / 40
    col = l % 40
    s[row][col] = '#'
  end

  s.each do |ss|
    puts ss.join
  end
end

def solve1(lines)
  acc = 0
  cycle = 1
  regx = 1

  lines.split("\n").each do |line|
    if cycle == 20 || (cycle - 20) % 40 == 0
      acc += cycle * regx
    end

    cycle += 1
    cmd, val = line.split(' ')

    next if cmd == 'noop'

    if cycle == 20 || (cycle - 20) % 40 == 0
      acc += cycle * regx
    end

    cycle += 1
    regx += val.to_i
  end

  puts acc
end

main