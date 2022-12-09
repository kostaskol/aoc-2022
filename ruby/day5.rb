require 'pry'

INSTR_REGEX = /move (.+) from (.+) to (.+)/.freeze

def parse_state(state)
  clean_state = state.gsub('    ', ' | ').gsub(/\[|\]/, '')
  data = clean_state.split("\n").map { |l| l.split(' ') }[0..-2]

  data.map do |l|
    l.map { |s| s == '|' ? '' : s }
  end
end

def parse_instructions(instr)
  res = Struct.new(:amount, :from, :to)
  instr.split("\n").map do |i|
    m = i.match(INSTR_REGEX).to_a[1..]

    res.new(*m.map(&:to_i))
  end
end

def get_and_remove_top_of_column(state, c)
  state.size.times do |t|
    unless (el = state[t][c]).empty?
      state[t][c] = ''
      return el
    end
  end
  binding.pry
end

def get_and_remove_top_of_column_2(state, c, amount)
  els = []
  curr_amount = 0
  state.size.times do |t|
    unless (el = state[t][c]).empty?
      state[t][c] = ''
      els << el
      curr_amount += 1

      return els if curr_amount == amount
    end
  end
end

def put_on_top_of_column(state, c, el)
  state.size.times.with_index do |t, indx|
    unless state[t][c].empty?
      if indx.zero?
        single_row = state[0].map { |_| '' }
        single_row[c] = el
        state.prepend(single_row)
      else
        state[indx - 1][c] = el
        return
      end
    end
  end

  state[-1][c] = el
end

def print_table(arr)
  width = arr.flatten.max.to_s.size + 2
end

def execute(state, instrs)
  indx = 0
  instrs.each do |i|
    indx += 1
    i.amount.times do |t|
      el = get_and_remove_top_of_column(state, i.from - 1)
      put_on_top_of_column(state, i.to - 1, el)
    end
  end
end

def execute_2(state, instrs)
  instrs.each do |i|
    els = get_and_remove_top_of_column_2(state, i.from - 1, i.amount)
    els.reverse.each do |e|
      put_on_top_of_column(state, i.to - 1, e)
    end
  end
end

file = File.read('input/day5.in')

state, instructions = file.split("\n\n")

s = parse_state(state)
i = parse_instructions(instructions)

execute_2(s, i)

foo = s[0].map.with_index do |_, i|
  get_and_remove_top_of_column(s, i)
end.to_a
