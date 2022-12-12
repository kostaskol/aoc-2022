require 'pry'
require 'rubygems'
require 'algorithms'

def main
  fname =
    if ARGV.any? { |a| a == '-t' }
      'input/test/day12.in'
    else
      'input/day12.in'
    end

  cont = File.read(fname)

  grid, start, target = to_grid(cont)

  if ARGV.any? { |a| a == '-e' }
    solve2(grid, start)
  else
    solve1(grid, start, target)
  end
end

def to_grid(cont)
  lines = cont.split("\n")
  dim_x = lines[0].size
  dim_y = lines.size
  start = [0, 0]
  target = [0, 0]

  grid = Array.new(dim_y) { |_| Array.new(dim_x) }

  lines.each_with_index do |row, y_indx|
    row.split('').each_with_index do |val, x_indx|
      grid_val =
        case val
        when 'S'
          start = [y_indx, x_indx]
          {
            value: 10_000,
            start: true,
            parent: nil,
            distance: 0
          }
        when 'E'
          target = [y_indx, x_indx]
          {
            value: 'z'.ord % 97,
            target: true,
            parent: nil,
            distance: nil
          }
        else
          {
            value: val.ord % 97,
            parent: nil,
            distance: nil
          }
        end

      grid[y_indx][x_indx] = grid_val
    end
  end

  [grid, start, target]
end

def possible_moves(grid, curr_row, curr_col)
  curr_val = grid[curr_row][curr_col]
  max_y = grid.size - 1
  max_x = grid[0].size - 1

  neighbours = [
    # Above
    [(curr_row - 1).clamp(0, max_y), curr_col],
    # Below
    [(curr_row + 1).clamp(0, max_y), curr_col],
    # Left
    [curr_row, (curr_col - 1).clamp(0, max_x)],
    # Right
    [curr_row, (curr_col + 1).clamp(0, max_x)]
  ].uniq

  neighbours.delete([curr_row, curr_col])

  neighbours.filter_map do |(y, x)|
    n_val = grid[y][x]

    next if (n_val[:value] - curr_val[:value]) > 1

    if n_val[:distance].nil? || (curr_val[:distance] + 1 < n_val[:distance])
      n_val.merge!(
        distance: curr_val[:distance] + 1,
        parent: [curr_row, curr_col]
      )

      [y, x]
    end
  end
end

def solve1(grid, start, target)
  curr = start
  to_visit = Containers::Heap.new(possible_moves(grid, curr[0], curr[1])) do |x, y|
    (x <=> y) == -1
  end

  while (curr = to_visit.pop)
    curr_val = grid[curr[0]][curr[1]]
    puts "Visiting #{curr_val}@#{curr}"
    possible_moves(grid, curr[0], curr[1]).each do |move|
      to_visit.push(move)
    end
  end

   #curr = [target[0], target[1]]
   #path = []
   #while !curr.nil?
   #  path << [curr[0], curr[1]]
   #  curr = grid[curr[0]][curr[1]][:parent]
   #end

   #vgrid = Array.new(grid.size) do |row|
   #  Array.new(grid[0].size) do |col|
   #    if grid[row][col][:value] == 10_000
   #      'S '
   #    else
   #      "#{grid[row][col][:value]} "
   #    end
   #  end
   #end

   #path.reverse.each do |curr|
   #  vgrid[curr[0]][curr[1]][1] = '|'

   #  vgrid.each do |row|
   #    puts row.join(',')
   #  end
   #  puts
   #end

   #grid.each do |row|
   #  puts row.map { |v| v[:value] }.join(',')
   #end

  puts "Target: #{grid[target[0]][target[1]][:distance]}"
end

main