require 'pry'
require 'json'

def main
  filename =
    if ARGV.any? { |a| a == '-t' }
      'input/test/day13.in'
    else
      'input/day13.in'
    end
  lines = File.read(filename)

  if ARGV.any? { |a| a == '-e' }
    solve2(lines)
  else
    solve1(lines)
  end
end

# -1 correct order
# 0 indecisive
# 1 wrong order
def cmp(first, second)
  if first.instance_of?(second.class)
    # Both numeric
    if first.is_a?(Numeric)
      first <=> second
    # Both arrays
    elsif first == second
      0
    else
      llen = [first.size, second.size].max

      llen.times do |indx|
        new_first = first[indx]
        new_second = second[indx]

        return -1 if new_first.nil?
        return 1 if new_second.nil?

        ord = cmp(new_first, new_second)

        return ord unless ord.zero?
      end
      0
    end
  elsif first.is_a?(Numeric)
    cmp([first], second)
  else
    cmp(first, [second])
  end
end

def solve1(lines)
  sols = []
  lines.split("\n\n").each_with_index do |group, indx|
    lines = group.split("\n").map { |l| JSON.parse(l) }

    sols << indx + 1 if cmp(*lines) == -1
  end

  puts sols.sum
end

def solve2(lines)
  # Way 1 (check positions)
  i2 = 1
  i6 = 2

  lines.split("\n").each do |line|
    next if line.empty?

    line = JSON.parse(line)
    if cmp(line, [[2]]) == -1
      i2 += 1
      i6 += 1
    elsif cmp(line, [[6]]) == -1
      i6 += 1
    end
  end

  puts i2 * i6

  # Way 2 (sort array)
  lines = lines.split("\n\n").flat_map do |group|
    group.split("\n").map { |l| JSON.parse(l) }
  end

  lines << [[2]]
  lines << [[6]]

  asc_sort = ->(a, b) { cmp(a, b) }
  lines = lines.sort(&asc_sort)

  res = lines.filter_map.with_index do |l, indx|
    indx + 1 if [[[2]], [[6]]].include?(l)
  end

  puts res.inject(:*)
end

main
