OP_REGEX = /new = (.+)\s(.)\s(.+)/.freeze

def main
  filename =
    if ARGV.select { |a| a == '-t' }.any?
      'input/test/day11.in'
    else
      'input/day11.in'
    end

  lines = File.read(filename).split("\n\n")
  monkeys =
    lines.each_with_object({}) do |monkey, acc|
      acc.merge!(parse_monkey(monkey))
    end

  if ARGV.select { |a| a == '-e' }.any?
    solve2(monkeys)
  else
    solve1(monkeys)
  end
end

def parse_monkey(monkey)
  lines = monkey.split("\n")

  # "Monkey N:" -> N
  id = lines[0].split(' ')[1][0...-1].to_i
  # "Starting items: 1, 2, 3" -> [1, 2, 3]
  items = lines[1].split(':')[1].split(',').map { |i| i.strip.to_i }
  # "Operation: new = old * 19" -> ['old', '*', 19]
  op = begin
    expr = lines[2].split(':')[1].strip
    expr.match(OP_REGEX)[1..]
  end
  # "Test: divisible by N" -> N
  t = lines[3].split(' ')[-1].to_i
  # "If true: throw to monkey N" -> N
  # "If false: throw to monkey M" -> M
  tr = lines[4].split(' ')[-1].to_i
  fa = lines[5].split(' ')[-1].to_i

  {
    id => {
      items: items,
      operation: op,
      test: {
        div_by: t,
        true => tr,
        false => fa
      },
      num_inspections: 0
    }
  }
end

def new_worry(operation, old_item)
  val_or_worry = ->(op, old) { op == 'old' ? old : op.to_i }

  lval = val_or_worry.call(operation[0], old_item)
  op = operation[1] == '*' ? :* : :+
  rval = val_or_worry.call(operation[2], old_item)

  lval.send(op, rval)
end

def solve1(monkeys)
  20.times do |i|
    monkeys.each do |(_, m)|
      # Abuse delete_if so we can mutate the array while iterating
      m[:items].delete_if do |item|
        m[:num_inspections] += 1

        final_worry = new_worry(m[:operation], item) / 3

        cond_applies = (final_worry % m.dig(:test, :div_by)).zero?
        new_monkey = m.dig(:test, cond_applies)
        monkeys.dig(new_monkey, :items) << final_worry

        true
      end
    end
  end

  puts monkeys.map { |_, v| v[:num_inspections] }.sort.last(2).inject(:*)
end

def solve2(monkeys)
  base_mod = monkeys.map { |_, m| m.dig(:test, :div_by) }.inject(:*)

  10_000.times do |i|
    monkeys.each do |(_, m)|
      # Abuse delete_if so we can mutate the array while iterating
      m[:items].delete_if do |item|
        m[:num_inspections] += 1

        final_worry = new_worry(m[:operation], item) % base_mod

        cond_applies = (final_worry % m.dig(:test, :div_by)).zero?
        new_monkey = m.dig(:test, cond_applies)
        monkeys.dig(new_monkey, :items) << final_worry

        true
      end
    end
  end

  puts monkeys.map { |_, v| v[:num_inspections] }.sort.last(2).inject(:*)
end

main