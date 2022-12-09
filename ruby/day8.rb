# Returns
# * Which axis should be followed (row, col),
# * How to get the next value for the axis (:-/:+)
# * A lambda regarding when the iteration should stop
def parametrize(grid, dir)
  if %i[above left].include?(dir)
    tmp = dir == :above ? :row : :col
    cond = ->(t) { t > - 1 }
    [tmp, :-, cond]
  elsif %i[below right].include?(dir)
    tmp = dir == :below ? :row : :col
    cond = ->(t) { t < grid.count }
    [tmp, :+, cond]
  else
    raise ArgumentError
  end
end

def visible_from(grid, row, col, dir)
  val = grid[row][col]
  broke = false

  tmp, mthd, cond = parametrize(grid, dir)

  if tmp == :row
    # Method is either - or +
    tmp_row = row.send(mthd, 1)
    while cond.call(tmp_row)
      if grid[tmp_row][col] >= val
        broke = true
        break
      end
      tmp_row = tmp_row.send(mthd, 1)
    end
  else
    tmp_col = col.send(mthd, 1)
    while cond.call(tmp_col)
      if grid[row][tmp_col] >= val
        broke = true
        break
      end
      tmp_col = tmp_col.send(mthd, 1)
    end
  end

  !broke
end

def scenic_score_towards(grid, row, col, dir)
  views = 0
  val = grid[row][col]

  tmp, mthd, cond = parametrize(grid, dir)

  if tmp == :row
    # Method is either - or +
    tmp_row = row.send(mthd, 1)
    while cond.call(tmp_row)
      views += 1
      break if grid[tmp_row][col] >= val

      tmp_row = tmp_row.send(mthd, 1)
    end
  else
    tmp_col = col.send(mthd, 1)
    while cond.call(tmp_col)
      views += 1
      break if grid[row][tmp_col] >= val

      tmp_col = tmp_col.send(mthd, 1)
    end
  end

  views
end

def count_visible(grid)
  # Grid is a square so there are <rows> * 4 trees
  # but we subtract 4 due to double counting the corners
  visible_trees = grid.count * 4 - 4

  (1..grid.count - 2).each do |row|
    (1..grid[row].count - 2).each do |col|
      next unless %i[above below left right].any? do |dir|
        visible_from(grid, row, col, dir)
      end

      visible_trees += 1
    end
  end

  puts visible_trees
end

def best_view(grid)
  best_view = 0

  # Trees on the outside will always have at least one 0 view
  # so there's no point in considering them
  (1..grid.count - 2).each do |row|
    (1..grid[row].count - 2).each do |col|
      scenic_score =
        %i[above below left right].map do |dir|
          scenic_score_towards(grid, row, col, dir)
        end.inject(&:*)

      best_view = scenic_score if scenic_score > best_view
    end
  end

  puts best_view
end

input_file =
  if ARGV.select { |a| a == '-t' }.any?
    'input/test/day8.in'
  else
    'input/day8.in'
  end

cont = File.read(input_file)
grid = cont.split("\n").map { |r| r.split('') }

if ARGV.select { |a| a == '-e' }.any?
  best_view(grid)
else
  count_visible(grid)
end
