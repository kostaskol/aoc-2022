require 'pry'
require 'set'

RE = /Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)/.freeze

def parse_input(cont)
  cont.split("\n").map do |line|
    md = RE.match(line)

    raise :foo if md.size != 5

    {
      s: [md[1], md[2]].map(&:to_i),
      b: [md[3], md[4]].map(&:to_i)
    }
  end
end

def main
  fname, loi, max =
    if ARGV.any? { |a| a == '-t' }
      ['input/test/day15.in', 10, 20]
    else
      ['input/day15.in', 2_000_000, 4_000_000]
    end

  cont = File.read(fname)

  inp = parse_input(cont)

  if ARGV.any? { |a| a == '-e' }
    solve2(inp, max)
  else
    solve1(inp, loi)
  end
end

def man_dist(x, y)
  (x[0] - y[0]).abs + (x[1] - y[1]).abs
end

def solve1(lines, loi)
  taken = Set.new

  beacons = []

  lines.each do |line|
    m_dist = man_dist(line[:s], line[:b])

    taken.add([line[:s][0], line[:s][1]]) if line[:s][1] == loi
    beacons << line[:b]

    loi_dist = (line[:s][1] - loi).abs

    next if loi_dist > m_dist

    # Number of columns on `loi` that will be occupied
    loi_taken = m_dist - loi_dist

    point = [line[:s][0], loi]
    taken.add(point)

    loi_taken.times do |loi_t|
      point1 = [line[:s][0] - (loi_t + 1), loi]
      point2 = [line[:s][0] + (loi_t + 1), loi]

      taken.add(point1)
      taken.add(point2)
    end
  end

  beacons.each do |b|
    taken.delete(b)
  end
end

def clamp(val, min, max)
  if val > max
    max
  elsif val < min
    min
  else
    val
  end
end

def solve2(lines, max)
  taken = Hash.new()

  indx = 0
  lines.each do |line|
    indx += 1
    print "\rCalculating ranges of scanner ##{indx}/#{lines.count}"
    m_dist = man_dist(line[:s], line[:b])

    if taken.include?(line[:b][1])
      taken[line[:b][1]] << (line[:b][0]..line[:b][0])
    else
      taken[line[:b][1]] = [line[:b][0]..line[:b][0]]
    end


    m_dist.times do |ld|
      max_x = clamp(line[:s][1] + (m_dist - ld), 0, max)
      min_x = clamp(line[:s][1] - (m_dist - ld), 0, max)

      loi1 = line[:s][0] + ld
      loi2 = line[:s][0] - ld

      [loi1, loi2].each do |loi|
        next unless loi >= 0 && loi <= max

        if taken.include?(loi)
          taken[loi] << (min_x..max_x)
        else
          taken[loi] = [(min_x..max_x)]
        end
      end
    end
  end

  puts
  max.times do |row|
    print "\rExamining row ##{row}/#{max}"
    ranges = collapse(taken[row], row == 3)

    gap = nil
    ranges.each do |r|
      if r.cover?(gap)
        gap = nil
      elsif r.begin > 0 && r.begin < max
        gap = r.begin - 1
      end

      if r.end < max && r.end > 0
        gap = r.end + 1
      end
    end

    if gap
      puts "\nSignal: #{row * 4_000_000 + gap}"
      break
    end
  end
end

def collapse(ranges, brk)
  finished = false
  new_ranges = ranges
  until finished
    prev_ranges_count = new_ranges.size
    new_ranges =
      new_ranges.sort_by(&:begin).each_slice(2).each_with_object([]) do |(range1, range2), acc|
        if range1.nil?
          if overlaps?(acc[-1], range2)
            l = acc.pop
            acc << collapse_range(l, range2)
          else
            acc << range2
          end
        elsif range2.nil?
          if overlaps?(acc[-1], range1)
            l = acc.pop
            acc << collapse_range(l, range1)
          else
            acc << range1
          end
        elsif overlaps?(range1, range2)
          acc << collapse_range(range1, range2)
        else
          acc << range1
          acc << range2
        end
      end

    finished = true if new_ranges.size == prev_ranges_count || new_ranges.size == 1
  end

  new_ranges
end

def collapse_range(r1, r2)
  beg = [r1.begin, r2.begin].min
  ending = [r1.end, r2.end].max

  (beg..ending)
end

def overlaps?(range1, range2)
  range2.begin <= range1.end && range1.begin <= range2.end
end

main
