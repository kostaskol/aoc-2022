# frozen_string_literal: true

MAX_AVAILABLE_SIZE = 70_000_000
MIN_SIZE_REQ = 30_000_000

# Modelifies a directory. Has a name and can contain both subdirectories
# (instances of Dir) as well as files. Each file has a size
class Dir
  attr_reader :name, :subdirs, :files

  def initialize(name)
    @name = name
    @subdirs = {}
    @files = {}
    @rec_size = 0
    @overall_size = 0
  end

  def add_subdir(subdir)
    sdc = Dir.new(subdir)
    @subdirs[subdir] = sdc
  end

  def add_file(file, size)
    @files[file] = size
  end

  def root?
    @name == '/'
  end

  def pprint(indentation = 0)
    ind = ' ' * indentation
    puts "#{ind}- #{name} (dir)"
    subdirs.each do |_, sd|
      sd.pprint(indentation + 2)
    end
    files.each do |f, size|
      puts "#{ind * 2}- #{f} (file, size=#{size})"
    end
  end

  def size
    acc = subdirs.map { |_, sd| sd.size }.sum
    total_size = acc + files.values.map(&:to_i).sum
    @overall_size = total_size
    @rec_size = total_size if total_size < 100_000
    total_size
  end

  def accumulate_solution1
    rec_sizes = subdirs.map { |_, sd| sd.accumulate_solution1 }.sum
    @rec_size + rec_sizes
  end

  def accumulate_sizes
    [@overall_size, *subdirs.map { |_, sd| sd.accumulate_sizes }.flatten]
  end

  def find_solution2(min_size)
    acc = accumulate_sizes
    acc.sort
    acc.select { |a| a > min_size }.first
  end
end

data = File.read('foo.in')

puts data
curr_dir = rd = Dir.new('/')
path = [curr_dir]
data.split("\n").each do |line|
  parts = line.split(' ')

  if parts[1] == 'cd'
    case parts[2]
    when '/'
      curr_dir = rd
      path = [rd]
    when '..'
      next if curr_dir.root?

      path.pop
      curr_dir = path.last
    else
      d = curr_dir.subdirs[parts[2]]
      curr_dir = d
      path << curr_dir
    end
  # output
  elsif parts[0] == 'dir'
    curr_dir.add_subdir(parts[1])
  else
    curr_dir.add_file(parts[1], parts[0])
  end
end

rd.pprint
# max_size = rd.size
# puts rd.accumulate_solution1
# puts rd.find_solution2(MIN_SIZE_REQ - (MAX_AVAILABLE_SIZE - max_size))
