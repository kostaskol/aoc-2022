require 'pry'
require 'rubygems'
require 'algorithms'

include Containers

def main
  fname =
    if ARGV.any? { |a| a == '-t' }
      'input/test/day16.in'
    else
      'input/day16.in'
    end

  cont = File.read(fname)
  graph = parse_graph(cont)

  if ARGV.any? { |a| a == '-e' }
    solve2(graph)
  else
    solve1(graph)
  end
end

def parse_graph(cont)
  re = /Valve ([A-Z]{2}) has flow rate=(\d+); tunnels lead to valves (.*)/

  cont.split("\n").each_with_object({}) do |line, graph|
    md = re.match(line)

    raise unless md

    s = md[1]
    fr = md[2].to_i
    neighb = md[3].split(',')

    graph[s] = {
      flow_rate: fr,
      neighbours: neighb,
      parent: nil,
      score: nil,
      steps: 0
    }
  end
end

def astar(graph, curr)
  cmp = ->(x, y) { y <=> x }
  pq = PriorityQueue.new(&cmp)

  graph.each do |_, val|
    val[:steps] = 1_000_000
  end

  graph['AA'][:steps] = 0
  visited = Set.new

  score = ->(node) { node[:flow_rate] - node[:steps] - 1 }
  pq.push(curr, score.call(curr))

  while (curr = pq.pop)
    curr[:neighbours].each do |neighbour|
      new_score = score.call(neighbour)

      if new_score < neighbour[:score]
        neighbour[:score] = new_score
      end
    end
  end

end

def solve1(graph)
  open_valves = []

  curr_node = graph['AA']
  30.times do |minute|
    minute += 1

    astar(graph, curr_node)
  end
end

main