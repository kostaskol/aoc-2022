data = File.read("input/day6.in").strip

indx = 13
data.split('').each_cons(14) do |e|
  indx += 1
  if e.uniq == e
    puts indx
    break
  end
end
