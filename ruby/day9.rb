require 'pry'
require 'set'

def main
  filename =
    if ARGV.select { |a| a == '-t' }.any?
      'input/test/day9.in'
    else
      'input/day9.in'
    end

  lines = File.read(filename).split("\n")

  if ARGV.select { |a| a == '-e' }.any?
    solve2(lines)
  else
    solve1(lines)
  end
end

def move(h, t)
  x_dist = (h[0] - t[0]).abs
  y_dist = (h[1] - t[1]).abs

  x_aligned = x_dist.zero?
  y_aligned = y_dist.zero?
  # Adjascent. No need to move
  return if x_dist <= 1 && y_dist <= 1

  if x_dist > 1 && y_aligned
    mthd = h[0] < t[0] ? :- : :+
    t[0] = t[0].send(mthd, 1)
  elsif y_dist > 1 && x_aligned
    mthd = h[1] < t[1] ? :- : :+
    t[1] = t[1].send(mthd, 1)
  else
    # move along y axis
    mthd = h[0] < t[0] ? :- : :+
    t[0] = t[0].send(mthd, 1)
    mthd = h[1] < t[1] ? :- : :+
    t[1] = t[1].send(mthd, 1)
  end
end

def solve2(lines)
  snake = Array.new(10) { |_| [0, 0] }
  visited = Set.new

  for line in lines
    dir, amount = line.split(' ')

    amount.to_i.times do |_|
      visited.add([snake.last[0], snake.last[1]])

      h = snake.first
      case dir
      when 'U'
        h[0] += 1
      when 'R'
        h[1] += 1
      when 'D'
        h[0] -= 1
      when 'L'
        h[1] -= 1
      else
        raise RuntimeError
      end

      snake.each_cons(2) do |(bh, t)|
        move(bh, t)
      end
    end
  end
  puts visited.count
end

def solve1(lines)
  h = [0, 0]
  t = [0, 0]
  visited = Set.new

  for line in lines
    dir, amount = line.split(' ')

    amount.to_i.times do |_|
      # Pass by ref shenanigans
      visited.add([t[0], t[1]])

      case dir
      when 'U'
        h[0] += 1
      when 'R'
        h[1] += 1
      when 'D'
        h[0] -= 1
      when 'L'
        h[1] -= 1
      else
        raise RuntimeError
      end
      move(h, t)
    end
    puts "#{dir}#{amount}: H:#{h},T:#{t}"
  end

  puts visited.count
end


main