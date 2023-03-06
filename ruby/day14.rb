require 'pry'
require 'set'

def main
  fname =
    if ARGV.any? { |a| a == '-t' }
      'input/test/day14.in'
    else
      'input/day14.in'
    end

  cont = File.read(fname)

  if ARGV.any? { |a| a == '-e' }
    solve2(cont)
  else
    solve1(cont)
  end
end

def get_blocked(lines)
  blocked = Set.new

  lines.split("\n").each do |line|
    line.split(' -> ').each_cons(2) do |first, second|
      first_x, first_y = first.split(',')
      second_x, second_y = second.split(',')

      first_x, second_x = [first_x, second_x].sort
      first_y, second_y = [first_y, second_y].sort

      (first_x..second_x).each do |x|
        (first_y..second_y).each do |y|
          blocked.add([x, y].map(&:to_i))
        end
      end
    end
  end

  blocked
end

def sand_next_moves(sand)
  # In order of priority
  [[sand[0], sand[1] + 1], [sand[0] - 1, sand[1] + 1], [sand[0] + 1, sand[1] + 1]]
end

def solve1(lines)
  blocked = get_blocked(lines)

  rested = 0
  max_y = blocked.map { |b| b[1] }.max

  answer_found = false
  until answer_found
    # Generate one sand
    sand_pos = [500, 0]

    moved = false
    next_moves = sand_next_moves(sand_pos)
    while next_moves
      moved = false
      next_moves.each do |nm|
        next if blocked.include?(nm)

        sand_pos = nm
        moved = true

        answer_found = sand_pos[1] >= max_y
        break
      end

      break if answer_found

      unless moved
        blocked.add(sand_pos)
        rested += 1
        break
      end
      next_moves = sand_next_moves(sand_pos)
    end
  end

  puts rested
end

def solve2(lines)
  blocked = get_blocked(lines)

  rested = 0
  floor = blocked.map { |b| b[1] }.max + 2
  answer_found = false

  until answer_found
    # Generate one sand
    sand_pos = [500, 0]

    moved = false
    next_moves = sand_next_moves(sand_pos)
    while next_moves
      moved = false
      next_moves.each do |nm|
        next if blocked.include?(nm) || nm[1] == floor

        sand_pos = nm
        moved = true
        break
      end

      unless moved
        blocked.add(sand_pos)
        rested += 1

        answer_found = sand_pos == [500, 0]
        break
      end
      break if answer_found

      next_moves = sand_next_moves(sand_pos)
    end
  end
  puts rested
end

main
