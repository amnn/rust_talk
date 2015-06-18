require 'benchmark'

def maybe_add(x)
  if x >= 1000
    nil
  else
    x + 1
  end
end

class NilClass
  def and_then
    nil
  end
end

class Object
  def and_then
    yield self
  end
end

def bench_add1
  res = 0
  2000.times do
    unless res.nil?
      res = maybe_add(res)
    end
  end
  res
end

def bench_add2
  res = 0
  2000.times do
    res = res.and_then { |x| maybe_add(x) }
  end
  res
end

include Benchmark
Benchmark.benchmark(CAPTION, 7, FORMAT, "ns/iter: ") do |bm|
  t = bm.report("add1") { 10_000.times { bench_add1 } }
  [t * (1_000_000_000 / 10_000)]
end

Benchmark.benchmark(CAPTION, 7, FORMAT, "ns/iter: ") do |bm|
  t = bm.report("add2") { 10_000.times { bench_add2 } }
  [t * (1_000_000_000 / 10_000)]
end

#               user     system      total        real
# add1      1.930000   0.010000   1.940000 (  1.956756)
# ns/iter:  193000.0     1000.0   194000.0 (  195675.6)

#               user     system      total        real
# add2      2.460000   0.010000   2.470000 (  2.485771)
# ns/iter:  246000.0     1000.0   247000.0 (  248577.1)
